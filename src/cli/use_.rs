use seahorse::{Command, Context};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::use_sdkit::set_sdkit_as_current;
use crate::cli::intercepting_errors;
use std::error::Error;

use log::info;

pub fn build_cli_use() -> Command {
    Command::new("use")
        .usage("use [sdk-name] [version]")
        .action(|c: &Context| { intercepting_errors(exec_use, |e| {1})(c); })
}

pub fn exec_use(c: &Context) -> Result<(), Box<dyn Error>> {
    let candidate_name = c.args[0].clone();
    let version = c.args[1].clone();
    let os_and_arch = get_current_os_and_arch();
    print_and_log_info!("Attempting to use {} {} {}", candidate_name, version, os_and_arch);
    set_sdkit_as_current(candidate_name, version)?;
    Ok(())
}