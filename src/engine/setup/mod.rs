use std::error::Error;
use std::path::Path;

#[cfg(target_family = "windows")]
use {
    winreg::enums::HKEY_LOCAL_MACHINE,
    winreg::RegKey,
    winreg::enums::*,
};

use log::{debug, info, error};
use string_error::new_err;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn setup_on_windows() -> Result<(), Box<dyn Error>> {
    #[cfg(target_family = "windows")]{

        let current_dir = std::env::current_dir()?;
        print_and_log_info!("Will install in current directory: {}", current_dir.to_str().unwrap());

        let path_to_startup_bat = Path::new(std::env::var("UserProfile").unwrap().as_str()).join("cmd-init.bat");
        if path_to_startup_bat.exists() {
            // TODO: need to implement append logic here. which may break existing users' setups.
            //  for now, we only create a new file and fail instead if it exists.
            return Err(new_err("CMD startup bat file already exists. Will not interfere."));
        }

        // HKEY_LOCAL_MACHINE\Software\Microsoft\Command Processor
        let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
        let path = Path::new("Software").join("Microsoft").join("Command Processor");
        let (key, disp) = hkcu.create_subkey(&path)?;

        match disp {
            REG_CREATED_NEW_KEY => { print_and_log_info!("Created - 'Command Processor' key didn't exist - weird."); }
            REG_OPENED_EXISTING_KEY => { debug!("Opened 'Command Processor' reg key"); }
        }

        if let Ok(v) = key.get_raw_value("AutoRun") {
            if !format!("{:?}", v).is_empty() {
                return Err(new_err(
                    "AutoRun key is already set in registry.\n\
                        Seems like you already have an autorun CMD script set up.\n\
                        Will not interfere. Please install manually.\n"
                ));
            }
        }

        if path_to_startup_bat.clone().exists() {
            return Err(new_err(
                "cmd-init.bat already exists. Will not interfere. Please install manually."
            ));
        }

        print_and_log_info!("Creating a CMD startup script...");
        let mut startup_file = File::create(path_to_startup_bat.clone())?;
        let data = format!("doskey dsdk={}\\dsdk.bat $*", current_dir.to_str().unwrap());
        startup_file.write_all(data.as_bytes())?;
        print_and_log_info!("Created in {}", path_to_startup_bat.to_str().unwrap());

        print_and_log_info!("Linking the CMD startup script via the Registry...");
        key.set_value("AutoRun", &(path_to_startup_bat.to_str().unwrap()))?;
        print_and_log_info!("Linked");

        return Ok(());
    }

    return Err(new_err("This binary was not built for Windows!"));
}

// only support bash for now, TODO
pub fn setup_on_linux_bash() -> Result<(), Box<dyn Error>> {

    let current_dir = std::env::current_dir()?;
    print_and_log_info!("Will install in current directory: {}", current_dir.to_str().unwrap());

    let path_to_startup_script = Path::new(std::env::var("HOME").unwrap().as_str()).join(".bashrc");
    if path_to_startup_script.exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path_to_startup_script.clone())?;

        let p = current_dir.to_str().unwrap();
        print_and_log_info!("Appending to {}", path_to_startup_script.clone().to_str().unwrap());
        if let Err(e) = writeln!(file, "[[ -s \"{}/dsdk-init.sh\" ]] && source \"{}/dsdk-init.sh\"", p, p) {
            return Err(Box::new(e));
        }
        print_and_log_info!("Successfully appended.");
    } else {
        print_and_log_error!("No .bashrc file. Are you using a different shell?\n\
        Please install the utility manually. The instructions can be found \
        in the docs folder.");
        return Err(new_err("Expected a .bashrc file in $HOME"))
    }

    Ok(())
}