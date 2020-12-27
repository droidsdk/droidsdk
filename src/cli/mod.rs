use std::error::Error;

#[macro_export]
macro_rules! print_and_log_info {
    ($($arg:tt)+) => (
        info!($($arg)+);
        println!($($arg)+);
    )
}

macro_rules! print_and_log_error {
    ($($arg:tt)+) => (
        error!($($arg)+);
        println!($($arg)+);
    )
}

pub(crate) mod list;
pub(crate) mod install;
pub(crate) mod remove;
pub(crate) mod use_;
pub(crate) mod whatis;
pub(crate) mod revert;
pub(crate) mod setup;
pub(crate) mod activity;

use string_error::new_err;
use seahorse::{Context, App};

use log::{error};

use std::sync::Mutex;

fn get_exec_name() -> Result<String, Box<dyn Error>> {
    let closure = || -> Option<String> {
        let v = std::env::current_exe().ok()?
            .file_name()?
            .to_str()?
            .to_owned();
        return Some(v);
    };
    return match closure() {
        Some(str) => Ok(str),
        None => Err(new_err("Failed to obtain current executable name"))
    }
}

lazy_static!(
    pub static ref EXIT_CODE : Mutex<i32> = Mutex::new(0);
);

pub fn intercepting_errors(a: fn(&Context) -> Result<(), Box<dyn Error>>, on_error: fn(Box<dyn Error>) -> i32) -> Box<dyn Fn(&Context) -> ()> {
    return Box::new( move |c: &Context| {
        let res = a(c);
        match res {
            Err(e) => {
                print_and_log_error!("Failure!");
                print_and_log_error!("{}",e);
                let exit_code = on_error(e);
                *(EXIT_CODE.lock()
                    .expect(&*format!("Could not report exit code nicely: exit code was {} but panicked instead",exit_code))) = exit_code;
            },
            _ => {}
        }
    })
}


use crate::cli::list::build_cli_list;
use crate::cli::install::build_cli_install;
use crate::cli::use_::build_cli_use;
use crate::cli::whatis::build_cli_whatis;
use crate::cli::revert::build_cli_revert;
use crate::cli::setup::build_cli_setup;
use crate::cli::remove::build_cli_remove;
use crate::cli::activity::build_cli_activity;

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
        .command(build_cli_remove())

        .command(build_cli_use())
        .command(build_cli_revert())

        .command(build_cli_activity());
}
