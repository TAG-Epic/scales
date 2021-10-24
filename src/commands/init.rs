use clap::ArgMatches;
use crate::config;
use crate::utils;
use std::env;

pub fn run(matches: ArgMatches) {
    let matches = matches.subcommand_matches("init").unwrap();
    let name = matches.value_of("name");
    let version = matches.value_of("version");
    let force = matches.is_present("force");

    {
        let config_file = config::get_config_file(env::current_dir().unwrap());
        if config_file != None && !force {
            utils::error("A scales project already exists in the current/parent dirs. If you really want to create one use --force");
        }
    }



    let config = config::Config{
        name: if let Some(name) = name {Some(name.to_string())} else {None},
        version: if let Some(version) = version {Some(version.to_string())} else {None},
        dependencies: Vec::new()};
    config::write_config(config);


}
