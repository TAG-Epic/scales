use crate::config;
use crate::utils;
use std::env;
pub async fn run() {
    {
        let config_file = config::get_config_file(env::current_dir().unwrap());
        if config_file == None {
            utils::error("Could not find a scalesfile!");
        }
    }
    let config = config::get_config();

    println!("{} version {}", config.name.unwrap_or("Unknown".to_string()), config.version.unwrap_or("unknown".to_string()));
    let mut deps = "".to_string();
    for dependency in config.dependencies {
        let name = match dependency {
            config::Dependency::Pypi(dep) => dep.name + &dep.version,
            config::Dependency::Git(dep) => {
                if let Some(tag) = dep.tag {
                    format!("{}#{}", dep.uri, tag)
                } else {
                    format!("{}@{}#{}", dep.uri, dep.branch, dep.commit.unwrap())
                }
            }
        };
        deps.push_str(&(name + " "));
    }
    println!("Dependencies: {}", deps);
}
