extern crate core;

use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use cpal::traits::StreamTrait;
use env_logger::Env;

use crate::cli::{Commands, ParsedArgs};
use crate::client::ClientHandler;
use crate::server::ServerHandler;

mod cli;
mod client;
mod device_selector;
mod server;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let cli = ParsedArgs::parse();

    let stream = match cli.command {
        Commands::Client(args) => ClientHandler::new(args)?.create_stream()?,
        Commands::Server(args) => ServerHandler::new(args)?.create_stream()?,
    };
    stream.play()?;
    sleep(Duration::from_secs(999999999999));
    Ok(())
}
