use seahorse::{Command, Flag, Context};
use crate::sdkman_api::candidates;
use crate::sdkman_api::candidates::fetch_candidates;
use crate::sdkman_api::versions::fetch_versions;
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::filesystem::get_installed_candidate_versions;

pub fn build_cli_list() -> Command {
    Command::new("list")
        .usage("list [sdk-name]")
        .action(exec_list)
}

pub fn exec_list(c: &Context) {
    if(c.args.len() > 0) {
        let candidate_name = c.args[0].clone();
        let os_and_arch = get_current_os_and_arch();
        let current_version = c.args[1].clone(); // TODO
        let installed_versions = get_installed_candidate_versions(candidate_name.clone());
        println!("Listing versions for [{}] {}", os_and_arch, candidate_name);
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