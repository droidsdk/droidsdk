use std::path::{Path, PathBuf};
use std::fs::{create_dir_all, File, set_permissions, Permissions};
use flate2::read::GzDecoder;
use tar::Archive;
use std::io::copy;
use zip::ZipArchive;

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

// y so complicatd?
pub fn unpack_zip_archive(path_to_archive: &Path, target_folder: &Path) -> PathBuf {
    let filename = path_to_archive.file_name().unwrap().to_str().unwrap().to_string();
    let file = File::open(&path_to_archive).unwrap();

    let mut archive = ZipArchive::new(file).unwrap();
    let unpack_to = target_folder.join(filename);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = unpack_to.join(file.sanitized_name()); // TODO use a non-deprecated method

        if (&*file.name()).ends_with('/') {
            create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                set_permissions(&outpath, Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    return unpack_to
}