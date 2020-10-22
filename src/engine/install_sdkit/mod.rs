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

pub fn install_sdkit(sdkit: String, version: String, os_and_arch: String) -> Result<(), Box<dyn Error>> {
    // TODO: execution of pre- and post-install hooks
    //  (do we actually want to run downloaded .sh scripts?)

    // TODO: way too much manual code here. need to go through crates.io and look for existing solutions

    let install_path = get_workdir_subpath(PathBuf::from("candidates").join(sdkit.clone()).join(version.clone())
        .to_str().unwrap().to_string());

    if install_path.exists() {
        return Err(new_err(&*format!("Candidate already installed (path {} exists)", install_path.to_str().unwrap())));
    }

    // call SDKMAN! API which redirects us to the direct download link for the specified SDKit archive
    let url = format!("https://api.sdkman.io/2/broker/download/{}/{}/{}",
                      sdkit, version, os_and_arch);
    println!("GET {}", url);
    let client = reqwest::blocking::ClientBuilder::new()
        .redirect(Policy::none()) // doing redirects manually
        .build().expect("failed to create client");
    let resp = client.execute(
        client.get(&url).build().unwrap()
    ).unwrap();
    let headers = resp.headers();

    // mkdirs ./app-name/archives
    ensure_dir_exists("archives".to_string())?;

    // follow the redirect
    if resp.status() != 302 {
        return Err(new_err(&*format!("Did not receive a redirect from SDKMAN! server (status code {})", resp.status())));
    }
    let redirect = headers.get("location").unwrap().to_str().unwrap().to_string();

    // TODO there's probably a crate that does this, and better:
    // obtain the filename from the URL we've been redirected to
    let filename = redirect // https://domain.com/endpoint/filename.ext?maybe=params
        .split("/").last() // filename.ext?maybe=params
        .unwrap().split("?").next() // filename.ext
        .unwrap().to_string();
    let dl_path = get_workdir_subpath("archives".to_string()).join(filename.to_string());
    let dl_path_ = dl_path.clone();

    if !dl_path.exists() {
        println!(
            "Downloading {} {} {}\n\
            from {}\n\
            to {}",
            sdkit, version, os_and_arch, redirect, dl_path.to_str().unwrap().to_string());
        download_archive(redirect, dl_path.clone())?;
        println!("Download finished");
    }else {
        println!("Archive already exists: {}", dl_path.to_str().unwrap());
    }

    let tmp_path = get_workdir_subpath("tmp".to_string());
    if tmp_path.exists() {
        println!("tmp dir exists - recreating (clearing)");
        remove_dir_all(tmp_path)?;
    }

    let unpack_path = get_workdir_subpath(
        PathBuf::from("tmp")
            .join(format!("{}_{}", sdkit, version))
            .to_str().unwrap().to_string()
    );
    create_dir_all(unpack_path.clone())?;
    println!("Unpacking file...");
    if filename.ends_with(".tar.gz") {
        unpack_tar_gz_archive(&dl_path_, &unpack_path)
            .expect(&*format!("Failed unpacking .tar.gz archive at {}", dl_path.to_str().unwrap()))
    } else if filename.ends_with(".zip") {
        unpack_zip_archive(&dl_path_, &unpack_path)
            .expect(&*format!("Failed unpacking .zip archive at {}", dl_path.to_str().unwrap()))
    } else {
        return Err(new_err(&*format!("File {} is of unknown filetype", filename)))
    };

    // note: this assumes that every candidate has a "flat" directory structure
    // that is, in ${candidate}_HOME there are many files and directories of various types
    // and not just a single directory (which would be quite weird)
    // so far all candidates i've looked at seem to follow this
    let unpacked_to_path = traverse_single_dirs(&unpack_path)?;

    println!("Copying from {} to {}...", unpacked_to_path.to_str().unwrap(), install_path.to_str().unwrap());

    create_dir_all(install_path.clone())?;
    #[cfg(target_family = "windows")] {
        // basically if the destination directory exists, Windows will not give us permissions
        // so we need to nuke it
        if install_path.clone().exists() && install_path.clone().is_dir() {
            std::fs::remove_dir(install_path.clone());
        }
    }
    std::fs::rename(unpacked_to_path, install_path)?;

    println!("Unpack complete.");

    Ok(())
}

fn download_archive(url: String, dl_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut resp = reqwest::blocking::get(&url)?;
    let filesize = resp.content_length().ok_or(new_err("Could not read Content Length"))?;
    let arc_filesize = Arc::new(filesize);

    let mut out = File::create(dl_path)?;

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
    // TODO this format is ugly and unwieldy. need to find a crate for this or write our own solution
    //  definitely need: %progress, MB progress, MB total, download speed
    while downloaded.load(Ordering::SeqCst) < *arc_filesize {
        println!("Downloading... {}/{}", downloaded.load(Ordering::SeqCst), *arc_filesize);
        thread::sleep(Duration::new(10, 0));
    }

    copy_thread.join().expect("Join on download thread failed");

    Ok(())
}