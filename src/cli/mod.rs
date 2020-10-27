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

pub(crate) mod root;
pub(crate) mod list;
pub(crate) mod install;
pub(crate) mod use_;
pub(crate) mod whatis;
pub(crate) mod revert;

use string_error::new_err;
use seahorse::{Context};

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

