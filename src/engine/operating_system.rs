use std::env::consts:: {
    OS, ARCH
};

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