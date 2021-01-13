use seahorse::{Command, Context, Flag, FlagType};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::use_sdkit::{set_sdkit_as_current, undo_set_sdkit_as_current, clear_path};
use crate::cli::intercepting_errors;
use std::error::Error;

use log::info;

pub fn build_cli_revert() -> Command {
    Command::new("revert")
        .usage("revert [sdk-name]")
        .action(|c: &Context| { intercepting_errors(exec_revert, |e| {1})(c); })
        .flag(
            Flag::new("all", FlagType::Bool)
                .usage("--all(-a) :: clear all dsdk entries in PATH")
                .alias("a"),
        )
}

pub fn exec_revert(c: &Context) -> Result<(), Box<dyn Error>> {
    if c.bool_flag("all") {
        print_and_log_info!("Removing ALL dsdk entries from PATH env var");
        clear_path()?;
    } else {
        let candidate_name = c.args[0].clone();
        print_and_log_info!("Reverting {}", candidate_name);
        undo_set_sdkit_as_current(candidate_name)?;
    }

    Ok(())
}