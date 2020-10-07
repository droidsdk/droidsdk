use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

pub struct SdkManCandidate {
    readable_name: String,
    version: String,
    homepage: String,
    description: String,
    pub candidate_name: String
}

impl Display for SdkManCandidate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} {} ({})\n\
        {}", self.candidate_name, self.readable_name, self.version, self.homepage, self.description)
    }
}

pub fn fetch_candidates() -> Result<Vec<Box<SdkManCandidate>>, Box<dyn Error>> {
    // TODO: extract URL (to config? to env var?)
    let url = "https://api.sdkman.io/2/candidates/list";
    let body = reqwest::blocking::get(url)
        .unwrap().text().unwrap();

    const HEADER_REGEX : &str = r"(?x)
        -+?\n                     # candidate separator
        (?P<readable_name>[^(]*?) # readable candidate name
        \(                        # version brackets open
        (?P<version>[^(]+?)       # version
        \)                        # version brackets close
        \s+?                      # many-many spaces
        (?P<homepage>\w[^\n]+?)   # homepage link
        \n\n                      # empty line
        (?P<desc>(?:.|\n)*?)      # description
        \n\n                      # empty line
        \s*?                      # many-many spaces, again
        \$\ sdk\ install\         # sdk install tip, we only need the
        (?P<name>[^\n]*?)\n       # actual candidate name";
    let regex = Regex::new(HEADER_REGEX).unwrap();
    return Ok(regex.captures_iter(&*body).map(|cap| {
       return Box::new(SdkManCandidate{
           readable_name: cap["readable_name"].to_string(),
           version: cap["version"].to_string(),
           homepage: cap["homepage"].to_string(),
           description: cap["desc"].to_string(),
           candidate_name: cap["name"].to_string()
       })
    }).collect());
}