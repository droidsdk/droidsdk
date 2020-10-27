use seahorse::{Command, Context};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::use_sdkit::{set_sdkit_as_current, undo_set_sdkit_as_current};
use crate::cli::intercepting_errors;
use std::error::Error;

use log::info;

pub fn build_cli_revert() -> Command {
    Command::new("revert")
        .usage("revert [sdk-name]")
        .action(|c: &Context| { intercepting_errors(exec_revert, |e| {1})(c); })
}

pub fn exec_revert(c: &Context) -> Result<(), Box<dyn Error>> {
    let candidate_name = c.args[0].clone();
    print_and_log_info!("Reverting {}", candidate_name);
    undo_set_sdkit_as_current(candidate_name)?;
    Ok(())
}