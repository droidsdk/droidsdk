use std::error::Error;

pub(crate) mod root;
pub(crate) mod list;
pub(crate) mod install;
pub(crate) mod use_;
pub(crate) mod whatis;

use string_error::new_err;
use seahorse::{Context};

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
                eprintln!("Failure!");
                eprintln!("{}",e);
                let exit_code = on_error(e);
                *(EXIT_CODE.lock()
                    .expect(&*format!("Could not report exit code nicely: exit code was {} but panicked instead",exit_code))) = exit_code;
            },
            _ => {}
        }
    })
}

