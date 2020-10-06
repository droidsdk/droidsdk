use seahorse::{Command, Flag, Context, FlagType};
use crate::sdkman_api::candidates;
use crate::sdkman_api::candidates::{fetch_candidates, SdkManCandidate};
use crate::sdkman_api::versions::{fetch_versions, fetch_versions_java};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::filesystem::get_installed_candidate_versions;
use std::ops::Deref;
use std::borrow::Borrow;

// nobody's ever gonna use this command, but, well, the functionality is there, so may as well
pub fn build_cli_whatis() -> Command {
    Command::new("whatis")
        .usage("whatis [sdk-name]")
        .action(exec_whatis)
}

pub fn exec_whatis(c: &Context) {
    if(c.args.len() > 0) {
        let candidate_name = c.args[0].clone();
        let result = fetch_candidates().unwrap();
        let candidate = result.into_iter().find(|it| it.candidate_name == candidate_name);
        match candidate {
            None => { println!("Candidate {} not found.", candidate_name) },
            Some(c) => { println!("{}", c) },
        }
    } else {
        println!("Not enough arguments - please enter the candidate identifier");
    }
}