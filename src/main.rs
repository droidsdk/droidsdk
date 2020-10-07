#[macro_use]
extern crate lazy_static;

mod cli;
mod sdkman_api;
mod engine;

use cli::root::build_cli_root;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = build_cli_root();
    app.run(args);
}
