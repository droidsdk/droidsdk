use seahorse::{App, Command, Context, Flag, FlagType, error::FlagError};
use crate::cli::get_exec_name;
use crate::cli::list::build_cli_list;
use crate::cli::install::build_cli_install;
use crate::cli::use_::build_cli_use;

pub fn build_cli_root() -> App {
    return App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .description("SDKMAN! ")
        .usage(get_exec_name().unwrap()+" [args]")
        .flag(
            Flag::new("test", FlagType::Int)
                .usage(get_exec_name().unwrap()+" [args] --test(-t)")
                .alias("t"),
        )
        .command(build_cli_list())
        .command(build_cli_install())
        .command(build_cli_use())
        .action(|c| println!("Hello, {:?}", c.args));
}