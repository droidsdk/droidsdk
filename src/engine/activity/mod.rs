use std::collections::HashMap;
use crate::engine::filesystem::get_workdir_subpath;
use std::fs::{read_dir, read_to_string};
use std::error::Error;
use json::JsonValue;
use std::path::{PathBuf, Path};

use log::debug;
use string_error::new_err;

pub struct EnvDependency {
    pub candidate: String,
    pub version: String
}

pub type EnvDepList = Vec<EnvDependency>;
pub type ProjectId = String;
pub type ActivityId = String;

pub struct Activity {
    pub id: ActivityId,
    pub per_project: HashMap<ProjectId, EnvDepList>
}

pub struct Project {
    pub id: ProjectId,
    pub per_activity: HashMap<ActivityId, EnvDepList>
}

pub fn get_global_activities() -> Result<HashMap<ActivityId, Box<Activity>>, Box<dyn Error>> {
    let activities_folder = get_workdir_subpath(PathBuf::from("config").join("activities").to_str().unwrap().to_string());
    let mut rv: HashMap<ActivityId, Box<Activity>> = HashMap::new();
    debug!("reading folder {}", activities_folder.to_str().unwrap().to_string());
    read_dir(activities_folder.clone()).expect("Could not read activities config directory").try_for_each(|it| -> Result<(), Box<dyn Error>> {
        let child_item = it.expect(&*format!("Could not read some files in directory {}",activities_folder.to_str().unwrap()));
        let child_item_metadata = child_item.metadata()
            .expect(&*format!("Could not read metadata for item {}",child_item.path().to_str().unwrap()));
        let extension : String = child_item.path().extension().unwrap_or("".as_ref()).to_str().unwrap().                                                                                                                                                                                                                                              to_string();
        if child_item_metadata.is_file() && extension == "jsonc" {
            debug!("begin reading file {}", child_item.path().to_str().unwrap().to_string());
            let mut contents = read_to_string(child_item.path())?;
            contents = filter_comments(contents);

            let json = json::parse(&*contents)?;
            json.entries().for_each(|activity_data| {
                let mut per_project_conf: HashMap<ProjectId, EnvDepList> = HashMap::new();
                activity_data.1["projectDeps"].entries().for_each(|project_conf| {
                    per_project_conf.insert(project_conf.0.to_string(), parse_env_dep_list(project_conf.1.clone()));
                });
                let activity = Activity {
                    id: activity_data.0.to_string(),
                    per_project: per_project_conf
                };
                debug!("successfully parsed activity {}", activity.id);
                rv.insert(activity.id.clone(), Box::new(activity));
            });
            debug!("read file {}", child_item.path().to_str().unwrap().to_string());
        }

        Ok(())
    })?;

    return Ok(rv);
}

pub fn get_project_conf(project_dir: PathBuf) -> Result<Option<Project>, Box<dyn Error>>{
    let project_conf_file = project_dir.join(".dsdk.jsonc");
    if !project_conf_file.exists() {
        return Ok(None);
    }

    let mut proj = Project {
        id: "".to_string(),
        per_activity: HashMap::new()
    };

    let mut contents = read_to_string(project_conf_file.clone())?;
    contents = filter_comments(contents);

    let mut json = json::parse(&*contents)?;
    proj.id = json["projectId"].take_string()
        .ok_or(new_err(&*format!("{} config invalid: no id", project_dir.to_str().unwrap().to_string())))?;
    json["activities"].entries().for_each(|activity_data| {
        proj.per_activity.insert(activity_data.0.to_string(), parse_env_dep_list(activity_data.1.clone()));
        debug!("successfully parsed activity {}", activity_data.0.to_string());
    });
    debug!("read file {}", project_conf_file.to_str().unwrap().to_string());

    return Ok(Some(proj));
}

pub fn filter_comments(file: String) -> String {
    return file.split("\n").filter(|it| {
        !it.trim_start().starts_with("//")
    }).collect::<Vec<&str>>().join("\n");
}

pub fn parse_env_dep_list(j: JsonValue) -> EnvDepList {
    return j.entries().map(|mut kv| {
        EnvDependency {
            candidate: kv.0.to_string(),
            version: kv.1.clone().take_string().unwrap()
        }
    }).collect();
}