use std::path::{Path, PathBuf};
use std::fs::{create_dir_all, File};
use flate2::read::GzDecoder;
use tar::Archive;

pub fn ensure_dir_exists(relative_path: String) {
    let path = get_workdir_subpath(relative_path);
    create_dir_all(path);
}

pub fn get_workdir_subpath(relative_path: String) -> PathBuf {
    let p = get_app_workdir_path();
    return Path::new(p.as_path()).join(relative_path)
}

pub fn get_app_workdir_path() -> PathBuf {
    let path_to_home = std::env::var("HOME").unwrap_or("/".to_string());
    return Path::new(&path_to_home).join(".droidsdk");
}

pub fn unpack_tar_gz_archive(path_to_archive: &Path, target_folder: &Path) -> PathBuf {
    //https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html
    let filename = path_to_archive.file_name().unwrap().to_str().unwrap().to_string();
    let tar_gz = File::open(path_to_archive).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(target_folder).unwrap();

    let mut unpacked_to_path = target_folder.join(filename);
    unpacked_to_path.set_extension(""); // - .tar
    unpacked_to_path.set_extension(""); // - .gz
    return unpacked_to_path.clone()
}