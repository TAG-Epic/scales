use clap::{App, load_yaml};
use tokio;
mod commands;
pub mod config;
pub mod utils;

#[tokio::main]
async fn main() {
    let config = load_yaml!("config.yml");
    let matches = App::from(config).get_matches();

    let http_client = reqwest::Client::new();

    match matches.subcommand_name().unwrap() {
        "add" => commands::add::run(matches, http_client).await,
        "install" => commands::install::run(http_client).await,
        "show" => commands::show::run().await,
        "init" => commands::init::run(matches),
        _ => panic!("Command not implemented")
    }
}
