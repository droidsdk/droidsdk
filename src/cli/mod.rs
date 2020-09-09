use std::error::Error;

pub(crate) mod root;
pub(crate) mod interactive;

use string_error::new_err;

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