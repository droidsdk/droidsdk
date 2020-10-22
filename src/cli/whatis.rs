use seahorse::{Command, Context};
use crate::sdkman_api::candidates::{fetch_candidates};
use string_error::new_err;
use std::error::Error;
use crate::cli::intercepting_errors;

use log::{info, error};

// nobody's ever gonna use this command, but, well, the functionality is there, so may as well
pub fn build_cli_whatis() -> Command {
    Command::new("whatis")
        .usage("whatis [sdk-name]")
        .action(|c: &Context| { intercepting_errors(exec_whatis, |e| {1})(c); })
}

pub fn exec_whatis(c: &Context) -> Result<(), Box<dyn Error>> {
    if c.args.len() > 0 {
        let candidate_name = c.args[0].clone();
        let result = fetch_candidates()?;
        let candidate = result.into_iter().find(|it| it.candidate_name == candidate_name);
        match candidate {
            None => { print_and_log_info!("Candidate {} not found.", candidate_name); },
            Some(c) => { print_and_log_info!("{}", c); },
        }
    } else {
        print_and_log_error!("Not enough arguments - please enter the candidate identifier");
    }

    Ok(())
}