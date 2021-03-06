use colored::*;
use std::process;
use std::path::{Path, PathBuf};
use file_lock::FileLock;
use std::env;
use clap::crate_version;
use crate::config;
use regex::Regex;

pub async fn get_pypi_info(package_name: &str, http_client: &reqwest::Client) -> serde_json::Value {
    let url = &format!("https://pypi.org/pypi/{}/json", package_name).to_string();
    let body = http_client.get(url)
        .header("User-Agent", format!("Scales v{}. (https://github.com/tag-epic/scales)", crate_version!()))
        .send().await.unwrap()
        .text().await.unwrap();
    serde_json::from_str(&body).unwrap()

}
pub async fn get_latest_pypi_release(package_name: &str, http_client: &reqwest::Client) -> String {
    let package_info = get_pypi_info(package_name, http_client).await;
    package_info
        .get("info").unwrap()
        .get("version").unwrap().as_str().unwrap().to_string()
}

pub fn error(error: &str) {
    eprintln!("{} {}", "Error:".to_string().red().bold(), error);
    process::exit(1);
}


pub fn state_path() -> PathBuf {
    Path::new(&(env::var("HOME").unwrap().to_string() + "/.local/share/scales")).to_path_buf()
}

pub fn modules_path() -> PathBuf {
    state_path().join("modules")
}

pub fn modules_lock() -> FileLock {
    FileLock::lock(modules_path().join("modules.lock").to_str().unwrap(), true, false).unwrap()
}

pub async fn create_dependency(name: String, http_client: &reqwest::Client) -> config::Dependency {
    let git_regex = Regex::new("^git@.*").unwrap();
    if git_regex.is_match(&name) {
        let dependency = config::GitDependency{uri: name.to_string(), branch: "master".to_string(), commit: None, tag: None};
        return config::Dependency::Git(dependency);
    } else {
        if name.contains("==") {
            let splitted = name.rsplit_once("==").unwrap();
            let package = splitted.0;
            let version = splitted.1;
            let dependency = config::PypiDependency{ name: package.to_string(), version: version.to_string() };
            config::Dependency::Pypi(dependency)
        } else {
            let package_version = get_latest_pypi_release(&name, &http_client).await;
            let dependency = config::PypiDependency{ name: name.to_string(), version: package_version };
            config::Dependency::Pypi(dependency)
        }
    }

}
