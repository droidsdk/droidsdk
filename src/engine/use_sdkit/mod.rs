use std::env::var;
use regex::Regex;
use crate::engine::filesystem::{get_workdir_subpath, write_new_env_var_value};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;
use string_error::new_err;

use log::{debug, info, error};

pub fn set_sdkit_as_current(sdkit: String, version: String) -> Result<(), Box<dyn Error>> {
    let path_to_sdkit = get_workdir_subpath(PathBuf::from("candidates").join(sdkit.clone()).join(version.clone()).to_str().unwrap().to_string());
    print_and_log_info!("Checking for sdkit in {:?}", path_to_sdkit);
    if !path_to_sdkit.exists() {
        return Err(new_err(&*format!("{} {} does not seem to be installed.", sdkit, version)));
    }

    print_and_log_info!("Removing current dsdk {} from PATH", sdkit);
    if let Err(some) = undo_set_sdkit_as_current(sdkit) {
        // TODO match a specific error
        print_and_log_error!("Failed to remove the candidate already in path. That is fine.");
    }

    let mut env_path = var("PATH")?;
    let new_path = path_to_sdkit.join("bin");
    debug!("Old PATH: {}", env_path);

    let mut backup_path_file = File::create(get_workdir_subpath("path_backup".to_string()))?;
    backup_path_file.write_all(env_path.clone().as_ref())?;

    let (fs_separator, path_separator) = get_fs_and_path_separator();

    env_path = format!("{}{}{}", &*new_path.to_str().unwrap().to_string(), path_separator, env_path);

    write_new_env_var_value("PATH".to_string(), env_path.clone());
    debug!("New PATH value will be: {}", env_path);

    Ok(())
}

pub fn undo_set_sdkit_as_current(sdkit: String) -> Result<(), Box<dyn Error>> {
    let mut env_path = var("PATH")?;
    debug!("Old PATH: {}", env_path);

    let mut backup_path_file = File::create(get_workdir_subpath("path_backup".to_string()))?;
    backup_path_file.write_all(env_path.clone().as_ref())?;

    let (fs_separator, path_separator) = get_fs_and_path_separator();

    let rel_path = PathBuf::from("candidates").join(sdkit.clone()).to_str().unwrap().to_string();
    let abs_path = get_workdir_subpath(rel_path);

    let mut found_amount = 0;
    let env_path = env_path.split(&path_separator).collect::<Vec<&str>>().into_iter()
        .filter(|it| {
            if it.starts_with(abs_path.to_str().unwrap()) {
                print_and_log_info!("Found {} in PATH, will remove", abs_path.to_str().unwrap());
                found_amount+=1;
                false
            } else {
                true
            }
    }).collect::<Vec<&str>>().join(&path_separator);

    if(found_amount > 1) {
        print_and_log_error!("Multiple dsdk installs of {} were found in PATH!", sdkit);
        print_and_log_error!("This is unexpected and likely an error condition");
        print_and_log_error!("dsdk will remove all of these entries, however, if that's incorrect \
        you can retrieve the old PATH from the backup file");
    }

    if(found_amount < 1) {
        print_and_log_error!("No dsdk installs of {} were found in PATH!", sdkit);
        return Err(new_err("dsdk revert: found_amount < 1"))
    }

    write_new_env_var_value("PATH".to_string(), env_path.clone());
    debug!("New PATH value will be: {}", env_path);

    Ok(())
}

fn get_fs_and_path_separator() -> (String, String) {
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
    return (fs_separator.to_string(), path_separator.to_string());
}