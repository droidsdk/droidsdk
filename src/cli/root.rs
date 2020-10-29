use seahorse::{App, Flag, FlagType};
use crate::cli::get_exec_name;
use crate::cli::list::build_cli_list;
use crate::cli::install::build_cli_install;
use crate::cli::use_::build_cli_use;
use crate::cli::whatis::build_cli_whatis;
use crate::cli::revert::build_cli_revert;
use crate::cli::setup::build_cli_setup;

pub fn build_cli_root() -> App {
    return App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .description("SDKMAN! ")
        .usage(get_exec_name().unwrap()+" [args]")

        .command(build_cli_setup())

        .command(build_cli_whatis())
        .command(build_cli_list())

        .command(build_cli_install())

        .command(build_cli_use())
        .command(build_cli_revert());
}