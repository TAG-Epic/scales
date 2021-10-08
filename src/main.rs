use clap::{App, load_yaml};
mod commands;
pub mod config;
pub mod utils;

fn main() {
    let config = load_yaml!("config.yml");
    let matches = App::from(config).get_matches();
    match matches.subcommand_name().unwrap() {
        "add" => commands::add::run(matches),
        _ => panic!("Command not implemented")
    }
}
