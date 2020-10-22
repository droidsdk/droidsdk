use seahorse::{Command, Flag, Context, FlagType};
use crate::sdkman_api::candidates::fetch_candidates;
use crate::sdkman_api::versions::{fetch_versions, fetch_versions_java};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::filesystem::get_installed_candidate_versions;
use crate::cli::intercepting_errors;
use std::error::Error;

use log::info;

pub fn build_cli_list() -> Command {
    Command::new("list")
        .usage("list [sdk-name]")
        .action(|c: &Context| { intercepting_errors(exec_list, |e| {1})(c); })
        .flag(
            Flag::new("mine", FlagType::Bool)
                .usage("--mine(-m) :: only display installed versions")
                .alias("m"),
        )
}

pub fn exec_list(c: &Context) -> Result<(), Box<dyn Error>> {
    if c.args.len() > 0 {
        let candidate_name = c.args[0].clone();
        let os_and_arch = get_current_os_and_arch();
        let current_version = c.args[1].clone(); // TODO
        let installed_versions = get_installed_candidate_versions(candidate_name.clone());
        print_and_log_info!("Listing versions for [{}] {}", os_and_arch, candidate_name);
        let mut versions = if candidate_name == "java" {
            fetch_versions_java(candidate_name, os_and_arch, current_version, Vec::from(installed_versions))?
        } else {
            fetch_versions(candidate_name, os_and_arch, current_version, Vec::from(installed_versions))?
        };
        if c.bool_flag("mine") {
            print_and_log_info!("Only installed versions will be listed (--mine flag)");
            versions = versions.into_iter().filter(|it| {
                it.installed || it.local_only || it.selected
            }).collect()
        }
        for v in versions {
            println!("{}", v)
        }
    } else {
        print_and_log_info!("Listing available SDKits...");
        println!("Use whatis [candidate] for more info");
        let result = fetch_candidates()?;
        for candidate in result {
            print_and_log_info!("{}", candidate.candidate_name);
        }
    }
    Ok(())
}