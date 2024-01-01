mod command;
mod db_connection;
mod runner;
mod schema;
mod utils;
mod server;

use clap::Parser;
use command::{Command, Opts};
use dotenvy::dotenv;
use runner::send;
use runner::start;
use server::app;
use tracing::Level;
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    let filter = filter::Targets::new()
    .with_target("tower_http::trace::on_response", Level::TRACE)
    .with_target("tower_http::trace::on_request", Level::TRACE)
    .with_target("tower_http::trace::make_span", Level::DEBUG)
    .with_default(Level::INFO);
    let tracing_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();

    dotenv().ok();
    let opt = Opts::parse();
    let command = opt.command.unwrap_or(Command::Run);
    match command {
        Command::Send => {send().await},
        Command::Run => start().await,
        Command::Serve => app::serve().await,
    }
}
