use std::env::var;
use regex::Regex;
use crate::engine::filesystem::{get_workdir_subpath, write_new_env_var_value};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;
use string_error::new_err;

use log::{debug, info};

pub fn set_sdkit_as_current(sdkit: String, version: String) -> Result<(), Box<dyn Error>> {
    let path_to_sdkit = get_workdir_subpath(PathBuf::from("candidates").join(sdkit.clone()).join(version.clone()).to_str().unwrap().to_string());
    print_and_log_info!("Checking for sdkit in {:?}", path_to_sdkit);
    if !path_to_sdkit.exists() {
        return Err(new_err(&*format!("{} {} does not seem to be installed.", sdkit, version)));
    }

    let env_path = var("PATH")?;
    let new_path = path_to_sdkit.join("bin");
    debug!("Old PATH: {}", env_path);

    let mut backup_path_file = File::create(get_workdir_subpath("path_backup".to_string()))?;
    backup_path_file.write_all(env_path.clone().as_ref())?;

    let fs_separator: &str;
    let path_separator: &str;
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
                print_and_log_info!("Replacing \n{}\nwith \n{}",it, new_path.to_str().unwrap());
                found = true;
                return new_path.to_str().unwrap().to_string()
            }
            return it.to_string();
        }).collect::<Vec<String>>().join(path_separator);

    if !found {
        print_and_log_info!("Candidate not in PATH - appending");
        env_path = format!("{}{}{}", &*new_path.to_str().unwrap().to_string(), path_separator, env_path);
    }

    write_new_env_var_value("PATH".to_string(), env_path.clone());
    debug!("New PATH value will be: {}", env_path);

    Ok(())
}