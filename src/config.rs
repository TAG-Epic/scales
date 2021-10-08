use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct PypiDependency {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitDependency {
    pub uri: String,
    pub branch: String,
    pub commit: Option<String>,
    pub tag: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all(deserialize = "snake_case", serialize = "snake_case"))]
pub enum Dependency {
    Pypi(PypiDependency),
    Git(GitDependency),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub dependencies: Vec<Dependency>,
}

pub fn get_config_file<'a>(directory: PathBuf) -> Option<PathBuf> {
    let config_file = directory.join(".scales.json");
    if config_file.exists() {
        return Some(config_file);
    }
    let parent = directory.parent()?.to_path_buf();
    return get_config_file(parent);
}
pub fn get_config() -> Config {
    let file_path = get_config_file(env::current_dir().unwrap());
    if file_path == None {
        return Config {
            dependencies: Vec::new(),
        };
    }
    let file_path = file_path.unwrap();
    let file_path_name = file_path.display();

    let file = match File::open(&file_path) {
        Err(why) => panic!("couldn't open {}: {}", file_path_name, why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect("Could not parse config file");
    println!("{:#?}", config);
    config
}

pub fn write_config(config: Config) {
    let mut file_path = get_config_file(env::current_dir().unwrap());
    if file_path == None {
        file_path = Some(env::current_dir().unwrap().join(".scales.json"));
    }
    let file_path = file_path.unwrap();

}
