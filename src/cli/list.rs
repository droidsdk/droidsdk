use seahorse::{Command, Flag, Context};
use crate::sdkman_api::candidates;
use crate::sdkman_api::candidates::fetch_candidates;

pub fn build_cli_list() -> Command {
    Command::new("list")
        .usage("list [sdk-name]")
        .action(exec_list)
}

pub fn exec_list(c: &Context) {
    if(c.args.len() > 0) {
        let candidate_name = c.args[0].clone();
        println!("Listing versions for {}", candidate_name);

    } else {
        println!("Listing available SDKits...");
        let result = fetch_candidates().unwrap();
        for candidate in result {
            println!("Candidate {}\n", candidate)
        }
    }
}