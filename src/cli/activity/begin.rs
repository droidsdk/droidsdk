use seahorse::{Command, Flag, Context, FlagType};
use crate::sdkman_api::candidates::fetch_candidates;
use crate::sdkman_api::versions::{fetch_versions, fetch_versions_java};
use crate::engine::operating_system::{get_current_os_and_arch, get_sdkit_version_in_use};
use crate::engine::filesystem::{get_installed_candidate_versions, get_app_workdir_path, get_workdir_subpath};
use crate::cli::intercepting_errors;
use std::error::Error;

use log::{debug, info};
use string_error::new_err;
use crate::engine::activity::get_global_activities;
use std::ops::Deref;
use crate::engine::use_sdkit::set_sdkit_as_current;
use std::fs::{OpenOptions, File};
use std::io::Write;

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
        debug!("keys: {}", activity_global.per_project.keys().map(|s| &**s).collect::<Vec<_>>().join(", "));
        let deps = activity_global.per_project.get(project).unwrap();
        deps.into_iter().try_for_each::<_, Result<(), Box<dyn Error>>>(|dependency| {
            let rv = set_sdkit_as_current(dependency.candidate.clone(), dependency.version.clone())?;
            Ok(())
        })?;
    } else {
        return Err(new_err("Please specify activity ID"));
    }
    Ok(())
}