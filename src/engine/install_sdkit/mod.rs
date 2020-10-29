use crate::engine::filesystem::{ensure_dir_exists, get_workdir_subpath, unpack_tar_gz_archive, unpack_zip_archive, traverse_single_dirs};
use std::fs::{File, create_dir_all, remove_dir_all};
use std::{thread};
use reqwest::redirect::Policy;
use std::time::Duration;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc};
use std::path::PathBuf;
use std::error::Error;
use string_error::new_err;
use std::thread::JoinHandle;

use log::{info, error};
use size_format::SizeFormatterBinary;

#[cfg(target_family = "unix")]
use {
    signal_hook::iterator::Signals,
    signal_hook::SIGINT,
};

pub fn install_sdkit(sdkit: String, version: String, os_and_arch: String) -> Result<(), Box<dyn Error>> {
    // TODO: execution of pre- and post-install hooks
    //  (do we actually want to run downloaded .sh scripts?)

    // TODO: way too much manual code here. need to go through crates.io and look for existing solutions

    let install_path = get_workdir_subpath(PathBuf::from("candidates").join(sdkit.clone()).join(version.clone())
        .to_str().unwrap().to_string());
    print_and_log_info!("Will install to {:?}", install_path);

    if install_path.exists() {
        return Err(new_err(&*format!("Candidate already installed (path {} exists)", install_path.to_str().unwrap())));
    }

    // call SDKMAN! API which redirects us to the direct download link for the specified SDKit archive
    let url = format!("https://api.sdkman.io/2/broker/download/{}/{}/{}",
                      sdkit, version, os_and_arch);
    print_and_log_info!("Calling {}", url);
    let client = reqwest::blocking::ClientBuilder::new()
        .redirect(Policy::none()) // doing redirects manually
        .build().expect("failed to create client");
    let resp = client.execute(
        client.get(&url).build().unwrap()
    ).unwrap();
    let headers = resp.headers();

    // follow the redirect
    if resp.status() != 302 {
        return Err(new_err(&*format!("Did not receive a redirect from SDKMAN! server (status code {})", resp.status())));
    }
    let redirect = headers.get("location").unwrap().to_str().unwrap().to_string();
    print_and_log_info!("Will download from {}", redirect);

    // mkdirs ./app-name/archives
    ensure_dir_exists("archives".to_string())?;

    // TODO there's probably a crate that does this, and better:
    // obtain the filename from the URL we've been redirected to
    let filename = redirect // https://domain.com/endpoint/filename.ext?maybe=params
        .split("/").last() // filename.ext?maybe=params
        .unwrap().split("?").next() // filename.ext
        .unwrap().to_string();
    let dl_path = get_workdir_subpath("archives".to_string()).join(filename.to_string());
    let dl_path_ = dl_path.clone();

    if !dl_path.exists() {
        print_and_log_info!(
            "Downloading {} {} {}\n\
            from {}\n\
            to {}",
            sdkit, version, os_and_arch, redirect, dl_path.to_str().unwrap().to_string());
        download_archive(redirect, dl_path.clone())?;
        print_and_log_info!("Download finished");
    }else {
        print_and_log_info!("Archive already exists: {}", dl_path.to_str().unwrap());
    }

    let tmp_path = get_workdir_subpath("tmp".to_string());
    if tmp_path.exists() {
        print_and_log_info!("tmp dir exists - recreating (clearing)");
        remove_dir_all(tmp_path)?;
    }

    let unpack_path = get_workdir_subpath(
        PathBuf::from("tmp")
            .join(format!("{}_{}", sdkit, version))
            .to_str().unwrap().to_string()
    );
    create_dir_all(unpack_path.clone())?;
    print_and_log_info!("Unpacking file...");
    if filename.ends_with(".tar.gz") {
        unpack_tar_gz_archive(&dl_path_, &unpack_path)?
    } else if filename.ends_with(".zip") {
        unpack_zip_archive(&dl_path_, &unpack_path)?
    } else {
        return Err(new_err(&*format!("File {} is of unknown filetype", filename)))
    };

    // note: this assumes that every candidate has a "flat" directory structure
    // that is, in ${candidate}_HOME there are many files and directories of various types
    // and not just a single directory (which would be quite weird)
    // so far all candidates i've looked at seem to follow this
    let unpacked_to_path = traverse_single_dirs(&unpack_path)?;

    print_and_log_info!("Copying from {} to {}...", unpacked_to_path.to_str().unwrap(), install_path.to_str().unwrap());

    create_dir_all(install_path.clone())?;
    #[cfg(target_family = "windows")] {
        // basically if the destination directory exists, Windows will not give us permissions
        // so we need to nuke it
        if install_path.clone().exists() && install_path.clone().is_dir() {
            std::fs::remove_dir(install_path.clone());
        }
    }
    std::fs::rename(unpacked_to_path, install_path)?;

    print_and_log_info!("Unpack complete.");

    Ok(())
}

fn download_archive(url: String, dl_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut resp = reqwest::blocking::get(&url)?;
    let filesize = resp.content_length().ok_or(new_err("Could not read Content Length"))?;
    let arc_filesize = Arc::new(filesize);

    let dl_path_ref = &dl_path; // Rust, stop it. Rust.

    let mut out = File::create(dl_path.clone())?;

    let downloaded = Arc::new(AtomicU64::new(0));

    // run the stream reading code in separate thread
    // this way we have 2 threads
    // one actually downloads and record exact progress

    // HEADS UP: the following code does not propagate errors meaningfully from the spawned threads
    // for whatever god-forsaken reason Rust's wonderful Error trait does not implement Send + Sync
    // which means it cannot be passed between threads
    // so far i have not been able to find a way to send errors between threads meaningfully,
    // and all i did is spend way too much time on this. so, whatever. we just panic completely instead.
    let copy_thread: JoinHandle<()>;
    {
        let downloaded = downloaded.clone();
        let arc_filesize = arc_filesize.clone();
        copy_thread = thread::spawn(move || {
            let mut download = || -> Result<(), Box<dyn Error>> {loop {
                    let mut buffer: Vec<u8> = Vec::new();
                    buffer.resize(500, 0);
                    let read = resp.read(&mut buffer)?;
                    out.write(&buffer[0..read])?;
                    downloaded.fetch_add(read as u64, Ordering::SeqCst);

                    if downloaded.load(Ordering::SeqCst) == *arc_filesize {
                        break;
                    }
                }
                Ok(())
            };

            download()
                .expect("Download failed");
        });
    };

    // the other (current) thread outputs this progress every second to the console
    // TODO need to check if there's a crate that maybe does a better job
    let mut last_downloaded = 0;
    let output_period = 10 /*seconds*/;
    let filesize_format = SizeFormatterBinary::new(*arc_filesize);
    #[cfg(target_family = "unix")]
    let signals = Signals::new(&[SIGINT])?;
    loop {
        let downloaded_local = downloaded.load(Ordering::SeqCst);
        if downloaded_local > *arc_filesize { break; }

        let percent = (downloaded_local as f64) / (*arc_filesize as f64) * 100.0;

        let downloaded_since_last_iteration = downloaded_local - last_downloaded;
        let bps = downloaded_since_last_iteration / output_period;

        print_and_log_info!("Downloading... {}B/{}B {:.2}% at {}B/s",
            SizeFormatterBinary::new(downloaded_local), filesize_format, percent, SizeFormatterBinary::new(bps));

        last_downloaded = downloaded_local;

        // doing this in a parallel thread would be cleaner but the moons of Jupiter aren't
        // aligned correctly rn for me to bother with Rust's threading
        #[cfg(target_family = "unix")] { // TODO for windows
            for sig in signals.pending() {
                if dl_path.exists() {
                    print_and_log_error!("Partially downloaded archive will be deleted.");
                    std::fs::remove_file(dl_path_ref);
                }
                return Err(new_err("Interrupted by SIGINT."));
            }
        }

        thread::sleep(Duration::new(output_period, 0));
    }

    copy_thread.join().expect("Join on download thread failed");

    Ok(())
}