use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::cmp::{max, Ordering};
use std::num::ParseIntError;

pub struct SdkManCandidateVersion {
    pub version: String,
    pub identifier: String,
    pub installed: bool,
    pub local_only: bool,
    pub selected: bool
}

impl Display for SdkManCandidateVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] -> v{} {} {} {}", self.identifier, self.version,
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
            identifier: cap["version"].to_string(),
            selected: cap.name("selected").is_some(),
            installed: cap.name("installed").is_some(),
            local_only: cap.name("local").is_some()
        })
    }).collect();
    result.sort_by(|a, b| a.version.cmp(&b.version));
    return Ok(result);
}

pub fn fetch_versions_java(sdkit: String, os_and_arch: String, current_ver: String, installed_vers: Vec<String>) -> Result<Vec<Box<SdkManCandidateVersion>>, Box<Error>> {
    // TODO: extract URL (to config? to env var?)
    let url = format!("https://api.sdkman.io/2/candidates/{}/{}/versions/list?current={}&installed={}",
                      sdkit, os_and_arch, current_ver, installed_vers.join(","));
    let body = reqwest::blocking::get(&url)
        .unwrap().text().unwrap();

    let lines_iter = body.split('\n').into_iter();
    let count = lines_iter.clone().count();
    let lines = lines_iter.take(count-6).skip(5).collect::<Vec<&str>>();

    let mut rv: Vec<Box<SdkManCandidateVersion>> = lines.into_iter().map( |line| {
        let csv : Vec<&str> = line.split("|").map(|it| it.trim()).collect();
        Box::new(SdkManCandidateVersion {
            version: csv[2].to_string(),
            identifier: csv[5].to_string(),
            selected: csv[1] == ">>>",
            installed: csv[4] == "installed",
            local_only: csv[4] == "local only"
        })
    }).collect();

    rv.sort_by(|a, b| comp_versions(a.version.clone(), b.version.clone()));

    return Ok(rv);
}

fn comp_versions(a: String, b: String) -> Ordering {
    if a.is_empty() && b.is_empty() {
        return a.cmp(&b)
    }


    let a_parts = extract_leading_version_chunk(a);
    let b_parts = extract_leading_version_chunk(b);

    let cmp = try_cmp_as_ints(a_parts.0, b_parts.0);

    if cmp == Ordering::Equal {
        return comp_versions(a_parts.1, b_parts.1);
    }

    return cmp;
}

fn try_cmp_as_ints(a: String, b: String) -> Ordering {
    let a_parsed: Result<i32, ParseIntError> = a.parse();
    let b_parsed: Result<i32, ParseIntError> = b.parse();

    if a_parsed.is_ok() && b_parsed.is_ok() {
        return a_parsed.unwrap().cmp(&b_parsed.unwrap())
    }

    return a.cmp(&b);
}

fn extract_leading_version_chunk(v: String) -> (String, String) {
    if v.len() == 0 {
        return (String::new(), String::new())
    }
    let dot_index = v.find('.').unwrap_or(v.len());
    let leading = v.chars().take(dot_index).collect::<String>();
    let trailing = v.chars().skip(dot_index+1).take(v.len()).collect::<String>();
    return (leading, trailing);
}