mod cli;
mod sdkman_api;

use cli::root::build_cli_root;

use std::env;
use seahorse::{App, Command, Context, Flag, FlagType, error::FlagError};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = build_cli_root();
    app.run(args);
}
