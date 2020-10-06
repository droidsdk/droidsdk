use seahorse::{Command, Flag, Context, FlagType};
use crate::sdkman_api::candidates;
use crate::sdkman_api::candidates::fetch_candidates;
use crate::sdkman_api::versions::{fetch_versions, fetch_versions_java};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::filesystem::get_installed_candidate_versions;
use std::ops::Deref;
use std::borrow::Borrow;

pub fn build_cli_list() -> Command {
    Command::new("list")
        .usage("list [sdk-name]")
        .action(exec_list)
        .flag(
            Flag::new("mine", FlagType::Bool)
                .usage("--mine(-m) :: only display installed versions")
                .alias("m"),
        )
}

pub fn exec_list(c: &Context) {
    if(c.args.len() > 0) {
        let candidate_name = c.args[0].clone();
        let os_and_arch = get_current_os_and_arch();
        let current_version = c.args[1].clone(); // TODO
        let installed_versions = get_installed_candidate_versions(candidate_name.clone());
        println!("Listing versions for [{}] {}", os_and_arch, candidate_name);
        let mut versions = if candidate_name == "java" {
            fetch_versions_java(candidate_name, os_and_arch, current_version, Vec::from(installed_versions)).unwrap()
        } else {
            fetch_versions(candidate_name, os_and_arch, current_version, Vec::from(installed_versions)).unwrap()
        };
        if c.bool_flag("mine") {
            println!("Only installed versions will be listed (--mine flag)");
            versions = versions.into_iter().filter(|it| {
                it.installed || it.local_only || it.selected
            }).collect()
        }
        for v in versions {
            println!("{}", v)
        }
    } else {
        println!("Listing available SDKits...");
        println!("Use whatis [candidate] for more info");
        let result = fetch_candidates().unwrap();
        for candidate in result {
            println!("{}", candidate.candidate_name)
        }
    }
}