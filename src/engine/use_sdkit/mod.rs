use std::env::var;
use regex::Regex;
use crate::engine::filesystem::{get_workdir_subpath, write_new_env_var_value};
use std::fs::File;
use std::io::Write;
use std::ops::Add;

pub fn set_sdkit_as_current(sdkit: String, version: String) {
    let path_to_sdkit = get_workdir_subpath(format!("candidates/{}/{}", sdkit, version));
    if !path_to_sdkit.exists() {
        panic!("FAILURE - Not installed!")
    }

    let env_path = var("PATH").expect("Failed to read $PATH");
    let new_path = path_to_sdkit.join("bin");

    let mut backup_path_file = File::create(get_workdir_subpath("path_backup".to_string()))
        .expect("Failed to create backup file for $PATH");
    backup_path_file.write_all(env_path.clone().as_ref());

    let sdkman_regex: &str = &*format!(r"\.sdkman/candidates/{}", sdkit);
    let sdkman_current_regex: &str = &*format!(r"\.sdkman/candidates/{}/current", sdkit);
    let droidsdk_regex: &str = &*format!(r"\.droidsdk/candidates/{}", sdkit);
    let r1 = Regex::new(sdkman_regex).unwrap();
    let r2 = Regex::new(sdkman_current_regex).unwrap();
    let r3 = Regex::new(droidsdk_regex).unwrap();

    let mut found = false;
    let mut env_path: String = env_path.split(":").collect::<Vec<&str>>().into_iter()
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
        }).collect::<Vec<String>>().join(":");

    if !found {
        println!("Candidate not in PATH - appending");
        env_path = format!("{}:{}", env_path, &*new_path.to_str().unwrap().to_string());
    }

    write_new_env_var_value("PATH".to_string(), env_path);
}