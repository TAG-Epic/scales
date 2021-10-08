use clap::ArgMatches;
use crate::config;
use crate::utils;
use regex::Regex;

pub fn run(matches: ArgMatches) {
    let matches = matches.subcommand_matches("add").unwrap();
    let packages = matches.values_of("packages").unwrap();

    let mut config = config::get_config();
    
    let http_regex = Regex::new("^http://.*").unwrap();
    let git_regex = Regex::new("^git@.*").unwrap();
    
    for package in packages {
        if http_regex.is_match(package) {
            panic!("Http dep not supported.");
        } else if git_regex.is_match(package) {
            let dependency = config::GitDependency{uri: package.to_string(), branch: "master".to_string(), commit: None, tag: None};
            config.dependencies.push(config::Dependency::Git(dependency));
        } else {
            if package.contains("|") {
                let splitted = package.rsplit_once("|").unwrap();
                let package = splitted.0;
                let version = splitted.1;
                let dependency = config::PypiDependency{ name: package.to_string(), version: version.to_string() };
                config.dependencies.push(config::Dependency::Pypi(dependency));
            } else {
                let package_version = utils::get_latest_pypi_release(package);
                let dependency = config::PypiDependency{ name: package.to_string(), version: package_version };
                config.dependencies.push(config::Dependency::Pypi(dependency));
            }
        }
    }
    println!("Package list: {:#?}", config);

}
