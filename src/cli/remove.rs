use seahorse::{Command, Context};
use crate::engine::install_sdkit::remove_sdkit;
use crate::engine::operating_system::get_current_os_and_arch;
use crate::cli::intercepting_errors;
use std::error::Error;

use log::info;

pub fn build_cli_remove() -> Command {
    Command::new("remove")
        .usage("remove [sdk-name]")
        .action(|c: &Context| { intercepting_errors(exec_remove, |e| {1})(c); })
}

pub fn exec_remove(c: &Context) -> Result<(), Box<dyn Error>> {
    let candidate_name = c.args[0].clone();
    let version = c.args[1].clone();
    let os_and_arch = get_current_os_and_arch();
    print_and_log_info!("Removing {} {} {}", candidate_name, version, os_and_arch);
    remove_sdkit(candidate_name, version, os_and_arch)?;
    Ok(())
}