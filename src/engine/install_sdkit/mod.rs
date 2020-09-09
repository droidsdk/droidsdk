use crate::engine::filesystem::{ensure_dir_exists, get_workdir_subpath};
use std::fs::{File, create_dir_all};
use std::{io, thread};
use reqwest::redirect::Policy;
use std::time::Duration;
use std::io::{Read, Write, IoSlice};
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;
use tar::Archive;
use flate2::read::GzDecoder;
use std::path::PathBuf;

pub fn install_sdkit(sdkit: String, version: String, os_and_arch: String) {
    // TODO: execution of pre- and post-install hooks
    //  (do we actually want to run downloaded .sh scripts?)

    // TODO: way too much manual code here. need to go through crates.io and look for existing solutions

    // call SDKMAN! API which
    // redirects us to the direct download link for the specified SDKit archive
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
    ensure_dir_exists("archives".to_string());

    // follow the redirect
    assert_eq!(resp.status(), 302, "Did not receive a redirect from SDKMAN! server");
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
        download_archive(redirect, dl_path);
        println!("Download finished");
    }else {
        println!("Archive already exists: {}", dl_path.to_str().unwrap());
    }

    println!("Unpacking file...");

    //https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html
    let tar_gz = File::open(dl_path_.clone()).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let unpack_path = get_workdir_subpath(format!("tmp/{}_{}", sdkit, version));
    archive.unpack(unpack_path.clone()).unwrap();

    let mut unpacked_to_path = unpack_path.join(filename);
    unpacked_to_path.set_extension(""); // - .tar
    unpacked_to_path.set_extension(""); // - .gz

    let install_path = get_workdir_subpath(format!("candidates/{}/{}", sdkit, version));
    println!("Copying from {} to {}...", unpacked_to_path.to_str().unwrap(), install_path.to_str().unwrap());
    create_dir_all(install_path.clone());
    std::fs::rename(unpacked_to_path, install_path).expect("failed to rename file");

    println!("Unpack complete.");
}

fn download_archive(url: String, dl_path: PathBuf) {
    let mut resp = reqwest::blocking::get(&url).expect("Failed to retrieve specified SDKit");
    let filesize = resp.content_length().unwrap_or(0);
    let arc_filesize = Arc::new(filesize);

    let mut out = File::create(dl_path).expect("failed to create file");

    let mut downloaded = Arc::new(AtomicU64::new(0));

    // run the stream reading code in separate thread
    // this way we have 2 threads
    // one actually downloads and record exact progress
    let copy_thread;
    {
        let downloaded = downloaded.clone();
        let arc_filesize = arc_filesize.clone();
        copy_thread = thread::spawn(move || {
            loop {
                let mut buffer: Vec<u8> = Vec::new();
                buffer.resize(500, 0);
                let read = resp.read(&mut buffer).expect("failed to read bytes while downloading");
                out.write(&buffer[0..read]).expect("failed to write bytes to file");
                downloaded.fetch_add(read as u64, Ordering::SeqCst);

                if downloaded.load(Ordering::SeqCst) == *arc_filesize {
                    break;
                }
            }
        });
    };
    // the other (current) thread outputs this progress every second to the console
    while downloaded.load(Ordering::SeqCst) < *arc_filesize {
        println!("Downloading... {}/{}", downloaded.load(Ordering::SeqCst), *arc_filesize);
        thread::sleep(Duration::new(10, 0));
    }
    copy_thread.join();
}