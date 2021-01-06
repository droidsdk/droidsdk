use crate::engine::activity::{ActivityId, ProjectId, get_project_conf, Project};
use crate::engine::filesystem::{get_workdir_subpath, create_symlink_to_dir};
use std::fs::{create_dir_all, remove_file, remove_dir};
use string_error::new_err;

use crate::engine::operating_system::{PATH_ENV_VAR};
use crate::engine::use_sdkit::{append_to_env_path, partial_sdkit_resolve, partial_sdkit_version_resolve, remove_sdkit_from_path};

use log::{info};
use std::error::Error;

pub fn setup_environment_for_project(activity_id: ActivityId, project_conf: Project) -> Result<(), Box<dyn Error>> {
    let selected_dir_path = get_workdir_subpath("selected".to_string());
    create_dir_all(selected_dir_path.clone());

    let mut env_path_mutex = PATH_ENV_VAR.lock().unwrap();
    let mut env_path = env_path_mutex.clone();

    if project_conf.per_activity.contains_key(&*activity_id) {
        let per_activity_conf = project_conf.per_activity.get(&*activity_id).unwrap();
        per_activity_conf.into_iter().try_for_each(|cv| -> Result<(), Box<dyn Error>> {
            let candidate = partial_sdkit_resolve(cv.candidate.clone())?;
            let version = partial_sdkit_version_resolve(candidate.clone(), cv.version.clone())?;

            env_path = remove_sdkit_from_path(candidate.clone(), env_path.clone())?;

            let symlink_path = selected_dir_path
                .join(format!("current.{}.{}", project_conf.id, candidate));
            let candidate_path = get_workdir_subpath("candidates".to_string())
                .join(candidate.clone())
                .join(version.clone())
                .join("bin");
            if !candidate_path.exists() {
                return Err(new_err(&*format!("{} {} is not installed!", candidate, version)))
            }

            if symlink_path.exists() {
                // TODO CRITICAL windows compat
                remove_file(symlink_path.clone())?;
            }

            create_symlink_to_dir(symlink_path.clone(), candidate_path)?;
            env_path = append_to_env_path(symlink_path.to_str().unwrap().to_string(), env_path.clone());
            Ok(())
        })?;
    } else {
        print_and_log_info!("Project has no configuration for activity {}", activity_id);
    }

    *env_path_mutex = env_path;

    Ok(())
}