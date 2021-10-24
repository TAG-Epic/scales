use clap::ArgMatches;
use crate::config;
use crate::utils;
use regex::Regex;

pub async fn run(matches: ArgMatches, http_client: reqwest::Client) {
    let matches = matches.subcommand_matches("add").unwrap();
    let packages = matches.values_of("packages").unwrap();

    let mut config = config::get_config();
    
    let http_regex = Regex::new("^http://.*").unwrap();
    
    for package in packages {
        if config::is_dep_set(package, &config.dependencies) {
            utils::error("Package has already been added.");
        }
        if http_regex.is_match(package) {
            utils::error("Http dep not supported.");
        }
        let dep = utils::create_dependency(package.to_string(), &http_client).await;
        config.dependencies.push(dep);
    }
    config::write_config(config);
}
