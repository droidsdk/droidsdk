use seahorse::{Command, Context};
use crate::engine::install_sdkit::install_sdkit;
use crate::engine::operating_system::get_current_os_and_arch;
use crate::cli::intercepting_errors;
use std::error::Error;

pub fn build_cli_install() -> Command {
    Command::new("install")
        .usage("install [sdk-name]")
        .action(|c: &Context| { intercepting_errors(exec_install, |e| {1})(c); })
}

pub fn exec_install(c: &Context) -> Result<(), Box<dyn Error>> {
    let candidate_name = c.args[0].clone();
    let version = c.args[1].clone();
    let os_and_arch = get_current_os_and_arch();
    println!("Installing {} {} {}", candidate_name, version, os_and_arch);
    install_sdkit(candidate_name, version, os_and_arch)?;
    Ok(())
}