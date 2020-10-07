use seahorse::{Command, Context};
use crate::sdkman_api::candidates::{fetch_candidates};

// nobody's ever gonna use this command, but, well, the functionality is there, so may as well
pub fn build_cli_whatis() -> Command {
    Command::new("whatis")
        .usage("whatis [sdk-name]")
        .action(exec_whatis)
}

pub fn exec_whatis(c: &Context) {
    if c.args.len() > 0 {
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