use seahorse::{Command, Context};
use crate::cli::intercepting_errors;
use std::error::Error;

use crate::engine::setup;

use log::info;
use string_error::new_err;
use crate::engine::setup::{setup_on_windows, setup_on_linux_bash};

pub fn build_cli_setup() -> Command {
    Command::new("setup")
        .usage("setup")
        .action(|c: &Context| { intercepting_errors(exec_setup, |e| {1})(c); })
}

pub fn exec_setup(c: &Context) -> Result<(), Box<dyn Error>> {
    print_and_log_info!("Will attempt to setup dsdk automatically...");

    let installation_variant = if c.args.len() > 0 {
        c.args[0].clone()
    } else {
        std::env::consts::OS.to_string()
    };

    match &*installation_variant {
        "linux" => {
            print_and_log_info!("Attempting to setup on linux");
            setup_on_linux_bash()?;
        },
        "windows" => {
            print_and_log_info!("Attempting to setup on windows");
            setup_on_windows()?;
        },
        _ => {
            return Err(new_err(&*format!("I don't know how to install on '{}'", installation_variant)))
        }
    };

    print_and_log_info!("Setup finished without errors");
    print_and_log_info!("Please now source from your terminal's startup script or just restart the terminal");
    Ok(())
}