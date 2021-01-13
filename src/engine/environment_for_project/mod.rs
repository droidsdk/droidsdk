use crate::engine::activity::{ActivityId, ProjectId, get_project_conf, Project};
use crate::engine::filesystem::{get_workdir_subpath, create_symlink_to_dir};
use std::fs::{create_dir_all, remove_file, remove_dir, read_to_string, canonicalize, OpenOptions, File};
use string_error::new_err;

use crate::engine::operating_system::{PATH_ENV_VAR};
use crate::engine::use_sdkit::{append_to_env_path, partial_sdkit_resolve, partial_sdkit_version_resolve, remove_sdkit_from_path};

use log::{info, debug};
use std::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Write;

pub fn setup_environment_for_project(activity_id: ActivityId, project_conf: Project) -> Result<HashMap<String, String>, Box<dyn Error>> {
    if project_conf.per_activity.contains_key(&*activity_id) {
        let per_activity_conf = project_conf.per_activity.get(&*activity_id).unwrap();
        let mut rv: HashMap<String, String> = HashMap::new();
        per_activity_conf.into_iter().try_for_each(|cv| -> Result<(), Box<dyn Error>> {
            let candidate = partial_sdkit_resolve(cv.candidate.clone())?;
            let version = partial_sdkit_version_resolve(candidate.clone(), cv.version.clone())?;
            let symlink_path = symlink_candidate(candidate.clone(), version, project_conf.id.clone())?;
            rv.insert(candidate.clone(), symlink_path);
            Ok(())
        })?;
        return Ok(rv);
    } else {
        print_and_log_info!("Project has no configuration for activity {}", activity_id);
    }

    Ok(HashMap::new())
}

pub fn symlink_candidate(candidate: String, version: String, project_id: ProjectId) -> Result<String, Box<dyn Error>> {
    let selected_dir_path = get_workdir_subpath("selected".to_string());
    create_dir_all(selected_dir_path.clone());

    let candidate = partial_sdkit_resolve(candidate.clone())?;
    let version = partial_sdkit_version_resolve(candidate.clone(), version.clone())?;

    let symlink_path = selected_dir_path
        .join(format!("current.{}.{}", project_id, candidate));
    let candidate_path = get_workdir_subpath("candidates".to_string())
        .join(candidate.clone())
        .join(version.clone());
    if !candidate_path.exists() {
        return Err(new_err(&*format!("{} {} is not installed!", candidate, version)))
    }

    if symlink_path.exists() {
        // TODO CRITICAL windows compat
        remove_file(symlink_path.clone())?;
    }

    create_symlink_to_dir(symlink_path.clone(), candidate_path)?;

    return Ok(symlink_path.to_str().unwrap().to_string());
}

pub fn update_candidates_in_path(paths: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let mut env_path_mutex = PATH_ENV_VAR.lock().unwrap();
    let mut env_path = env_path_mutex.clone();

    paths.into_iter().try_for_each(|candidate_version_path| -> Result<(), Box<dyn Error>> {
        let candidate = candidate_version_path.0;
        let path_to_version =  candidate_version_path.1;
        let path_to_bin = PathBuf::from(path_to_version).join("bin").to_str().unwrap().to_string();
        env_path = remove_sdkit_from_path(candidate.clone(), env_path.clone())?;
        env_path = append_to_env_path(path_to_bin, env_path.clone());
        Ok(())
    })?;

    *env_path_mutex = env_path;

    Ok(())
}

pub fn add_to_watched_projects(project_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let watched_projects_path = get_workdir_subpath("watched_projects".to_string());
    let currently_watched = if watched_projects_path.exists() {
      read_to_string(watched_projects_path.clone())?
    } else {
        File::create(watched_projects_path.clone())?;
        "".to_string()
    };

    let mut already_present = false;
    for path in currently_watched.split("\n").into_iter() {
        if !path.is_empty() && canonicalize(project_path.clone())?.starts_with(path) {
            already_present = true;
            break;
        }
    }

    if already_present {
        debug!("Project is already watched: {}", project_path.to_str().unwrap().to_string());
        return Ok(())
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(watched_projects_path.clone())?;

    debug!("Adding project to watched: {}", project_path.to_str().unwrap().to_string());
    writeln!(file, "{}", project_path.to_str().unwrap().to_string())?;

    file.flush()?;

    Ok(())
}