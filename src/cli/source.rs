use seahorse::{Command, Context};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::use_sdkit::{set_sdkit_as_current, undo_set_sdkit_as_current};
use crate::cli::intercepting_errors;
use std::error::Error;

use crate::engine::activity::{get_project_conf, Project};

use log::info;
use std::fs::{File, read_to_string};
use crate::engine::filesystem::get_workdir_subpath;
use std::collections::HashMap;
use crate::engine::environment_for_project::setup_environment_for_project;

pub fn build_cli_source() -> Command {
    Command::new("source")
        .usage("source") // TODO allow passing in project id manually
        .action(|c: &Context| { intercepting_errors(exec_source, |e| {1})(c); })
}

pub fn exec_source(c: &Context) -> Result<(), Box<dyn Error>> {

    let current_activity = read_to_string(get_workdir_subpath("current_activity".to_string()))
        .unwrap_or("default".to_string());

    print_and_log_info!("Current activity: {}", current_activity);

    let mut project = Project {
        id: "global".to_string(),
        per_activity: HashMap::new()
    };

    let pwd = std::env::current_dir()?;
    print_and_log_info!("PWD {}", pwd.to_str().unwrap().to_string());
    let maybe_project_conf = get_project_conf(pwd)?;
    if let Some(project_conf) = maybe_project_conf {
        project = project_conf;
    }

    print_and_log_info!("Sourcing environment for project {}", project.id);
    setup_environment_for_project(current_activity, project)?;
    Ok(())
}