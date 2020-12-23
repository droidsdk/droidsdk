use seahorse::{Command, Context, Flag, FlagType};
use crate::engine::install_sdkit::{install_sdkit, InstallSDKitRequest};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::cli::intercepting_errors;
use std::error::Error;

use log::info;

pub fn build_cli_install() -> Command {
    Command::new("install")
        .usage("install [sdk-name]")
        .flag(
            Flag::new("download", FlagType::Bool)
                .usage("--download(-D) :: force downloading of package, even if an archive exists")
                .alias("D")
        )
        .action(|c: &Context| { intercepting_errors(exec_install, |e| {1})(c); })
}

pub fn exec_install(c: &Context) -> Result<(), Box<dyn Error>> {
    let candidate_name = c.args[0].clone();
    let version = c.args[1].clone();
    let os_and_arch = get_current_os_and_arch();
    print_and_log_info!("Installing {} {} {}", candidate_name, version, os_and_arch);

    let accept_existing_archive = !c.bool_flag("download");
    install_sdkit(InstallSDKitRequest { sdkit: candidate_name, version, os_and_arch, accept_existing_archive })?;
    Ok(())
}