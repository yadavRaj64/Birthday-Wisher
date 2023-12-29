mod command;
mod db_connection;
mod runner;
mod schema;
mod utils;

use clap::Parser;
use command::{Command, Opts};
use runner::send;
use runner::start;

#[tokio::main]
async fn main() {
    let opt = Opts::parse();
    let command = opt.command.unwrap_or(Command::Run);
    match command {
        Command::Send { to, subject, body } => send(to, subject, body).await,
        Command::Run => start().await,
    }
}
