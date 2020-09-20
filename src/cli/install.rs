use seahorse::{Command, Flag, Context};
use crate::sdkman_api::candidates::fetch_candidates;
use crate::sdkman_api::versions::fetch_versions;
use crate::engine::install_sdkit::install_sdkit;
use crate::engine::operating_system::get_current_os_and_arch;

pub fn build_cli_install() -> Command {
    Command::new("install")
        .usage("install [sdk-name]")
        .action(exec_install)
}

pub fn exec_install(c: &Context) {
    let candidate_name = c.args[0].clone();
    let version = c.args[1].clone();
    let os_and_arch = get_current_os_and_arch();
    println!("Installing {} {} {}", candidate_name, version, os_and_arch);
    install_sdkit(candidate_name, version, os_and_arch);
}