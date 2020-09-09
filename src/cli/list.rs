use seahorse::{Command, Flag, Context};
use crate::sdkman_api::candidates;
use crate::sdkman_api::candidates::fetch_candidates;
use crate::sdkman_api::versions::fetch_versions;

pub fn build_cli_list() -> Command {
    Command::new("list")
        .usage("list [sdk-name]")
        .action(exec_list)
}

pub fn exec_list(c: &Context) {
    if(c.args.len() > 0) {
        let candidate_name = c.args[0].clone();
        // TODO this is just a PoC. in future we need to fetch these from filesystem and/or env:
        let os_and_arch = c.args[1].clone();
        let current_version = c.args[2].clone();
        let installed_versions = &c.args[3..];
        println!("Listing versions for {}", candidate_name);
        let versions =
            fetch_versions(candidate_name, os_and_arch, current_version, Vec::from(installed_versions)).unwrap();
        for v in versions {
            println!("{}", v)
        }
    } else {
        println!("Listing available SDKits...");
        let result = fetch_candidates().unwrap();
        for candidate in result {
            println!("Candidate {}\n", candidate)
        }
    }
}