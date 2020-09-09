use std::path::{Path, PathBuf};
use std::fs::create_dir_all;

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