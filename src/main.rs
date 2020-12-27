#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate json;

#[macro_use]
mod cli;
mod sdkman_api;
mod engine;

use std::env;
use std::process::exit;
use crate::cli::build_cli_root;
use engine::operating_system::PATH_ENV_VAR;
use crate::engine::filesystem::write_new_env_var_value;
use log::{debug, info};

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

    let new_path = PATH_ENV_VAR.lock().unwrap();
    write_new_env_var_value("PATH".to_string(), new_path.clone());
    debug!("New PATH value will be: {}", new_path);

    exit(error_code);
}
