use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

pub struct SdkManCandidateVersion {
    version: String,
    installed: bool,
    local_only: bool,
    selected: bool
}

impl Display for SdkManCandidateVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.version,
            if self.installed { "installed" } else { "" },
            if self.local_only { "installed locally" } else { "" },
            if self.selected { "selected" } else { "" })
    }
}

pub fn fetch_versions(sdkit: String, os_and_arch: String, current_ver: String, installed_vers: Vec<String>) -> Result<Vec<Box<SdkManCandidateVersion>>, Box<Error>> {
    // TODO: extract URL (to config? to env var?)
    let url = format!("https://api.sdkman.io/2/candidates/{}/{}/versions/list?current={}&installed={}",
                      sdkit, os_and_arch, current_ver, installed_vers.join(","));
    let body = reqwest::blocking::get(&url)
        .unwrap().text().unwrap();

    const HEADER_REGEX : &str = r"(?x)
        (?:(?P<selected>>)|\s)                   # maybe `> selected` otherwise space
        \s                                       # space
        (?:(?P<installed>\*)|(?P<local>\+)|\s)   # maybe `* installed` or `+ locally installed` otherwise space
        \s                                       # space
        (?P<version>\w[^\ ]*)                    # version identifier";
    let regex = Regex::new(HEADER_REGEX).unwrap();
    let mut result: Vec<Box<SdkManCandidateVersion>> = regex.captures_iter(&*body).map(|cap| {
        return Box::new(SdkManCandidateVersion{
            version: cap["version"].to_string(),
            selected: cap.name("selected").is_some(),
            installed: cap.name("installed").is_some(),
            local_only: cap.name("local").is_some()
        })
    }).collect();
    result.sort_by(|a, b| a.version.cmp(&b.version));
    return Ok(result);
}