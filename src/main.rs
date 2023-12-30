mod command;
mod db_connection;
mod runner;
mod schema;
mod utils;

use clap::Parser;
use command::{Command, Opts};
use dotenvy::dotenv;
use runner::send;
use runner::start;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let opt = Opts::parse();
    let command = opt.command.unwrap_or(Command::Run);
    match command {
        Command::Send => {send().await},
        Command::Run => start().await,
    }
}
