use seahorse::{Command, Flag, Context, FlagType};
use crate::sdkman_api::candidates::fetch_candidates;
use crate::sdkman_api::versions::{fetch_versions, fetch_versions_java};
use crate::engine::operating_system::{get_current_os_and_arch, get_sdkit_version_in_use};
use crate::engine::filesystem::{get_installed_candidate_versions, get_app_workdir_path, get_workdir_subpath};
use crate::cli::intercepting_errors;
use std::error::Error;

use log::{debug, info};
use string_error::new_err;
use crate::engine::activity::{get_global_activities, get_project_conf, Project};
use std::ops::Deref;
use crate::engine::use_sdkit::set_sdkit_as_current;
use std::fs::{OpenOptions, File, read_to_string};
use std::io::Write;
use crate::engine::environment_for_project::{setup_environment_for_project, update_candidates_in_path, symlink_candidate};
use std::path::PathBuf;
use std::collections::HashMap;

pub fn build_cli_begin() -> Command {
    Command::new("activity-begin")
        .usage("activity-begin [activity-id]")
        .action(|c: &Context| { intercepting_errors(exec_begin, |e| {1})(c); })
}

pub fn exec_begin(c: &Context) -> Result<(), Box<dyn Error>> {
    if c.args.len() > 0 {
        let activity = c.args[0].clone();

        let mut current_activity_file = File::create(get_workdir_subpath("current_activity".to_string()))?;
        current_activity_file.write_all(activity.as_ref());

        let conf = get_global_activities()?;



        // TODO: obtain current project here
        let project = "global";
        let activity_global = conf.get(&*activity).unwrap();
        let deps = activity_global.per_project.get(project).unwrap();
        let mut candidate_paths: HashMap<String, String> = HashMap::new();
        deps.into_iter().try_for_each::<_, Result<(), Box<dyn Error>>>(|dependency| {
            let rv = symlink_candidate(dependency.candidate.clone(), dependency.version.clone(), project.to_string())?;
            candidate_paths.insert(dependency.candidate.clone(), rv);

            Ok(())
        })?;
        update_candidates_in_path(candidate_paths)?;

        let watched_projects_path = get_workdir_subpath("watched_projects".to_string());
        let currently_watched = if watched_projects_path.exists() {
            read_to_string(watched_projects_path.clone())?
        } else { "".to_string() };
        currently_watched.split("\n").into_iter()
            .filter(|it| { !it.is_empty() } )
            .try_for_each(|path| -> Result<(), Box<dyn Error>> {
                let maybe_project_conf = get_project_conf(PathBuf::from(path))?;
                if let Some(project_conf) = maybe_project_conf {
                    setup_environment_for_project(activity.clone(), project_conf)?;
                    print_and_log_info!("Setting up for project {}", path);
                } else {
                    print_and_log_info!("Watched project {} does not exist or is missing the config file.", path);
                }

                Ok(())
            })?;
    } else {
        return Err(new_err("Please specify activity ID"));
    }
    Ok(())
}