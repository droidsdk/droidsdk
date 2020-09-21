use std::path::{Path, PathBuf};
use std::fs::{create_dir_all, File, set_permissions, Permissions, read_dir};
use flate2::read::GzDecoder;
use tar::Archive;
use std::io::{copy, Write};
use zip::ZipArchive;
use std::error::Error;

use string_error::new_err;
use std::sync::Mutex;

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

pub fn get_installed_candidate_versions(candidate: String) -> Vec<String> {
    let path = get_workdir_subpath(format!("candidates/{}",candidate.clone()));
    if !path.exists() {
        return vec![]
    }
    let mut rv: Vec<String> = Vec::new();
    read_dir(path.clone()).expect(&*format!("Could not read {} candidate directory",candidate)).for_each(|it| {
        let child_item = it.expect(&*format!("Could not read some files in directory {}",path.to_str().unwrap()));
        let child_item_metadata = child_item.metadata()
            .expect(&*format!("Could not read metadata for item {}",child_item.path().to_str().unwrap()));
        if child_item_metadata.is_dir() {
            rv.push(child_item.file_name().to_str().unwrap().to_string());
        }
    });
    return rv;
}

/// Traverses the chain of nested directories until arrives at a directory with stuff in it
/// Returns that directory's path
///
/// "With stuff in it" in this context means a directory which contains at least 1 file or
/// at least 2 directories
pub fn traverse_single_dirs(path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let mut file_count = 0;
    let mut dir_count = 0;
    let mut nested_dir_path : Option<PathBuf> = None;
    read_dir(path).expect(&*format!("Could not read directory {}", path.to_str().unwrap())).for_each(|it| {
        let child_element = it.expect(&*format!("Could not read some files in directory {}", path.to_str().unwrap()));
        let child_element_meta = child_element
            .metadata().expect(&*format!("Could not read metadata for file {}", child_element.path().to_str().unwrap()));
        if child_element_meta.is_dir() {
            nested_dir_path = Some(child_element.path());
            dir_count+=1;
        } else if child_element_meta.is_file() {
            file_count+=1;
        }
    });
    if file_count == 0 {
        if dir_count == 0 {
            Err(new_err(&*format!("Directory {} is empty", path.to_str().unwrap())))
        } else if dir_count == 1 {
            return traverse_single_dirs(&*nested_dir_path.expect("Found a directory but unable to read its path"));
        } else {
            return Ok(path.to_path_buf())
        }
    } else {
        return Ok(path.to_path_buf())
    }
}

pub fn unpack_tar_gz_archive(path_to_archive: &Path, target_folder: &Path) -> Result<(), Box<dyn Error>>  {
    //https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html
    let filename = path_to_archive.file_name().unwrap().to_str().unwrap().to_string();
    let tar_gz = File::open(path_to_archive).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(target_folder).unwrap();

    return Ok(())
}

// y so complicatd?
pub fn unpack_zip_archive(path_to_archive: &Path, target_folder: &Path) -> Result<(), Box<dyn Error>> {
    let filename = path_to_archive.file_name().unwrap().to_str().unwrap().to_string();
    let file = File::open(&path_to_archive).unwrap();

    let mut archive = ZipArchive::new(file).unwrap();
    let unpack_to = target_folder;

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

    return Ok(())
}

lazy_static!(
    static ref SETVARS_FILE : Mutex<File> = Mutex::new(File::create(get_workdir_subpath("setvars.sh".to_string()))
        .expect("Failed to overwrite setvars.sh"));
);
pub fn write_new_env_var_value(name: String, value: String) {
    SETVARS_FILE.lock()
        .expect("Failed to obtain mutex")
        .write_all(format!("export {}={}\n", name, value).as_ref())
        .expect("Failed to write new env var value");
}