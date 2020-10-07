use seahorse::{Command, Context};
use crate::engine::operating_system::get_current_os_and_arch;
use crate::engine::use_sdkit::set_sdkit_as_current;

pub fn build_cli_use() -> Command {
    Command::new("use")
        .usage("use [sdk-name] [version]")
        .action(exec_use)
}

pub fn exec_use(c: &Context) {
    let candidate_name = c.args[0].clone();
    let version = c.args[1].clone();
    let os_and_arch = get_current_os_and_arch();
    println!("Attempting to use {} {} {}", candidate_name, version, os_and_arch);
    set_sdkit_as_current(candidate_name, version)
        .expect("Could not set the specified SDKit as current");
}