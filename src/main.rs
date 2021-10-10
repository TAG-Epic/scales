use clap::{App, load_yaml};
use tokio;
mod commands;
pub mod config;
pub mod utils;

#[tokio::main]
async fn main() {
    let config = load_yaml!("config.yml");
    let matches = App::from(config).get_matches();
    match matches.subcommand_name().unwrap() {
        "add" => commands::add::run(matches),
        "install" => commands::install::run().await,
        _ => panic!("Command not implemented")
    }
}
