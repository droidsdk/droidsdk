#[macro_use]
extern crate lazy_static;

#[macro_use]
mod cli;
mod sdkman_api;
mod engine;

use cli::root::build_cli_root;

use std::env;
use std::process::exit;

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        // .chain(std::io::stdout()) // STDOUT is used for CLI-user communication
        .chain(fern::log_file("output.log").expect("Cannot create logfile"))
        .apply()
        .expect("Could not setup logger");

    log::info!("-------------------");
    log::info!("DSDK binary invoked");

    let args: Vec<String> = env::args().collect();
    let app = build_cli_root();
    app.run(args);
    let error_code = *cli::EXIT_CODE.lock()
        .expect("Could not read error code");
    exit(error_code);
}
