use std::env::consts:: {
    OS, ARCH
};
use std::error::Error;
use std::env::var;

use log::{info, error};

// returns a string specifying the operating system and architecture
// in SDKMAN!'s specific format
pub fn get_current_os_and_arch() -> String {
    let os = match OS {
        "linux" => "Linux",
        "windows" => "cygwin",
        _ => "OTHER_"
    };
    let arch = match ARCH {
        "x86" /*i686?*/ => "32",
        "aarch64" => "ARM64",
        _ => "64"
    };
    return format!("{}{}",os,arch);
}

pub fn get_fs_and_path_separator() -> (String, String) {
    let fs_separator: &str;
    let path_separator: &str;
    cfg_if::cfg_if! {
        if #[cfg(target_family="unix")] {
            fs_separator = r"/";
            path_separator = ":";
        } else {
            fs_separator = r"\\";
            path_separator = ";";
        }
    };
    return (fs_separator.to_string(), path_separator.to_string());
}

pub fn get_sdkit_version_in_use(sdkit: String) -> Result<Option<String>, Box<dyn Error>> {
    let mut env_path = var("PATH")?;
    let (fs_separator, path_separator) = get_fs_and_path_separator();

    let r = regex::Regex::new(&*format!("{}{}(?P<ver>[^{}]*)", sdkit, fs_separator, fs_separator)).unwrap();
    for path in env_path.split(&path_separator).collect::<Vec<&str>>().into_iter() {
        if let Some(cap) = r.captures(path) {
            let ver = cap.name("ver").unwrap().as_str().to_string();
            return Ok(Some(ver))
        }
    }

    Ok(None)
}