use seahorse::{App, Command, Context, Flag, FlagType, error::FlagError};
use crate::cli::get_exec_name;
use crate::cli::interactive::build_cli_interactive;
use crate::cli::list::build_cli_list;

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
        .command(build_cli_interactive())
        .command(build_cli_list())
        .action(|c| println!("Hello, {:?}", c.args));
}