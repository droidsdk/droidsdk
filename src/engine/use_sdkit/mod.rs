use std::env::var;
use regex::Regex;
use crate::engine::filesystem::{get_workdir_subpath, write_new_env_var_value};
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::PathBuf;

pub fn set_sdkit_as_current(sdkit: String, version: String) {
    let path_to_sdkit = get_workdir_subpath(PathBuf::from("candidates").join(sdkit.clone()).join(version.clone()).to_str().unwrap().to_string());
    if !path_to_sdkit.exists() {
        panic!("FAILURE - Not installed!")
    }

    let env_path = var("PATH").expect("Failed to read $PATH");
    let new_path = path_to_sdkit.join("bin");

    let mut backup_path_file = File::create(get_workdir_subpath("path_backup".to_string()))
        .expect("Failed to create backup file for $PATH");
    backup_path_file.write_all(env_path.clone().as_ref());

    let mut fs_separator = "/";
    let mut path_separator = ":";
    cfg_if::cfg_if! {
        if #[cfg(target_family="unix")] {
            fs_separator = r"/";
            path_separator = ":";
        } else {
            fs_separator = r"\\";
            path_separator = ";";
        }
    };

    let sdkman_regex: &str = &*format!(r"\.sdkman{}candidates{}{}", fs_separator, sdkit, fs_separator);
    let sdkman_current_regex: &str = &*format!(r"\.sdkman{}candidates{}{}{}current", fs_separator, fs_separator, sdkit, fs_separator);
    let droidsdk_regex: &str = &*format!(r"\.droidsdk{}candidates{}{}", fs_separator, fs_separator, sdkit);
    let r1 = Regex::new(sdkman_regex).unwrap();
    let r2 = Regex::new(sdkman_current_regex).unwrap();
    let r3 = Regex::new(droidsdk_regex).unwrap();

    let mut found = false;
    let mut env_path: String = env_path.split(path_separator).collect::<Vec<&str>>().into_iter()
        .map(|it| {
            if !found && (
                r1.is_match(it) ||
                    r2.is_match(it) ||
                    r3.is_match(it)
            ) {
                println!("Replacing \n{}\nwith \n{}",it, new_path.to_str().unwrap());
                found = true;
                return new_path.to_str().unwrap().to_string()
            }
            return it.to_string();
        }).collect::<Vec<String>>().join(path_separator);

    if !found {
        println!("Candidate not in PATH - appending");
        env_path = format!("{}{}{}", &*new_path.to_str().unwrap().to_string(), path_separator, env_path);
    }

    write_new_env_var_value("PATH".to_string(), env_path);
}