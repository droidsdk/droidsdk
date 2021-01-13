use std::env::var;
use regex::Regex;
use crate::engine::filesystem::{get_workdir_subpath, write_new_env_var_value, get_installed_candidates, get_installed_candidate_versions, get_app_workdir_path};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;
use string_error::new_err;

use log::{debug, info, error};
use crate::engine::operating_system::{get_fs_and_path_separator, PATH_ENV_VAR};
use std::ops::Deref;

pub fn set_sdkit_as_current(sdkit: String, version: String) -> Result<(), Box<dyn Error>> {
    let sdkit = partial_sdkit_resolve(sdkit)?;
    let version = partial_sdkit_version_resolve(sdkit.clone(), version)?;

    let path_to_sdkit = get_workdir_subpath(PathBuf::from("candidates").join(sdkit.clone()).join(version.clone()).to_str().unwrap().to_string());
    print_and_log_info!("Checking for sdkit in {:?}", path_to_sdkit);
    if !path_to_sdkit.exists() {
        return Err(new_err(&*format!("{} {} does not seem to be installed.", sdkit, version)));
    }

    let mut env_path = PATH_ENV_VAR.lock().unwrap();;
    debug!("Old PATH: {}", env_path);

    print_and_log_info!("Removing current dsdk {} from PATH", sdkit);
    match remove_sdkit_from_path(sdkit, env_path.clone()) {
        Ok(new_path) => {
            *env_path = new_path;
        },
        Err(error) => {
            // TODO match a specific error
            // TODO are we sure this is always non-critical?
            print_and_log_error!("Failed to remove the candidate already in path. \n\
            This is not a critical error.");
        },
    }
    debug!("PATH after removing: {}", env_path.clone());

    let new_path = path_to_sdkit.join("bin");

    let mut backup_path_file = File::create(get_workdir_subpath("path_backup".to_string()))?;
    backup_path_file.write_all(env_path.clone().as_ref())?;

    *env_path = append_to_env_path(new_path.to_str().unwrap().to_string(), (*env_path).clone());

    Ok(())
}

pub fn undo_set_sdkit_as_current(sdkit: String) -> Result<(), Box<dyn Error>> {
    // technically we don't need this here (the regex handles it anyway), but just to be consistent:
    let sdkit = partial_sdkit_resolve(sdkit)?;

    let mut env_path = PATH_ENV_VAR.lock().unwrap();;
    debug!("Old PATH: {}", env_path);

    let mut backup_path_file = File::create(get_workdir_subpath("path_backup".to_string()))?;
    backup_path_file.write_all(env_path.clone().as_ref())?;

    *env_path = remove_sdkit_from_path(sdkit, (*env_path).clone())?;
    debug!("New PATH value will be: {}", env_path);

    Ok(())
}

pub fn remove_sdkit_from_path(sdkit: String, path_value: String) -> Result<String, Box<dyn Error>> {
    let (_, path_separator) = get_fs_and_path_separator();

    let rel_path = PathBuf::from("candidates").join(sdkit.clone()).to_str().unwrap().to_string();
    let abs_path = get_workdir_subpath(rel_path);

    let mut found_amount = 0;
    let new_path_value = path_value.split(&path_separator).collect::<Vec<&str>>().into_iter()
        .filter(|it| {
            if it.starts_with(abs_path.to_str().unwrap()) {
                print_and_log_info!("Found {} in PATH, will remove", abs_path.to_str().unwrap());
                found_amount+=1;
                false
            } else {
                true
            }
        }).collect::<Vec<&str>>().join(&path_separator);

    if found_amount > 1 {
        print_and_log_error!("Multiple dsdk installs of {} were found in PATH!", sdkit);
        print_and_log_error!("This is unexpected and likely an error condition");
        print_and_log_error!("dsdk will remove all of these entries, however, if that's incorrect \
        you can retrieve the old PATH from the backup file");
    }

    if found_amount < 1 {
        print_and_log_error!("No dsdk installs of {} were found in PATH!", sdkit);
    }

    Ok(new_path_value)
}

pub fn clear_path() -> Result<(), Box<dyn Error>> {
    let mut env_path = PATH_ENV_VAR.lock().unwrap();;
    debug!("Old PATH: {}", env_path);

    let mut backup_path_file = File::create(get_workdir_subpath("path_backup".to_string()))?;
    backup_path_file.write_all(env_path.clone().as_ref())?;

    let (_, path_separator) = get_fs_and_path_separator();

    let app_path = get_app_workdir_path();

    let mut found_amount = 0;
    let new_path_value = (*env_path).split(&path_separator).collect::<Vec<&str>>().into_iter()
        .filter(|it| {
            if it.starts_with(app_path.to_str().unwrap()) {
                print_and_log_info!("Found {} in PATH, will remove", app_path.to_str().unwrap());
                found_amount+=1;
                false
            } else {
                true
            }
        }).collect::<Vec<&str>>().join(&path_separator);

    print_and_log_info!("Removed {} entries from PATH", found_amount);

    debug!("New PATH value will be: {}", new_path_value);
    *env_path = new_path_value;

    Ok(())
}

pub fn append_to_env_path(v: String, env_path: String) -> String {
    let (_, path_separator) = get_fs_and_path_separator();
    return format!("{}{}{}", v, path_separator, env_path);
}

pub fn partial_sdkit_resolve(sdkit_partial: String) -> Result<String, Box<dyn Error>> {
    let matches : Vec<String> = get_installed_candidates()
        .into_iter()
        .filter(|it| { it.starts_with(&sdkit_partial) })
        .collect();

    if matches.len() == 0 {
        return Err(new_err(&*format!("No candidate matched {}", sdkit_partial)));
    }
    if matches.len() > 1 {
        return Err(new_err(&*format!("Multiple candidates matched {}: \n {}", sdkit_partial, matches.join("\n"))));
    }

    let v = matches[0].clone();
    if sdkit_partial != v {
        print_and_log_info!("Resolved candidate \"{}\" to {}", sdkit_partial, v);
    }

    return Ok(v);
}

pub fn partial_sdkit_version_resolve(sdkit: String, version_partial: String) -> Result<String, Box<dyn Error>> {
    let matches : Vec<String> = get_installed_candidate_versions(sdkit)
        .into_iter()
        .filter(|it| { it.starts_with(&version_partial) })
        .collect();

    if matches.len() == 0 {
        return Err(new_err(&*format!("No candidate versions matched {}", version_partial)));
    }
    if matches.len() > 1 {
        return Err(new_err(&*format!("Multiple candidate versions matched {}: \n{}", version_partial, matches.join("\n"))));
    }

    let v = matches[0].clone();
    if version_partial != v {
        print_and_log_info!("Resolved version {} to {}", version_partial, v);
    }

    return Ok(v);
}