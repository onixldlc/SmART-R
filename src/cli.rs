use clap::{Args, Parser, Subcommand};
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ParsedArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the server
    Server(HandlerArgs),

    /// Run the client
    Client(HandlerArgs),
}

#[derive(Args, Debug)]
pub struct HandlerArgs {
    pub address: IpAddr,

    /// Optional port incase the user wants to change it
    #[arg(short, long, default_value = "34567")]
    pub port: u16,

    /// Optional device id incase the user wants to change it
    #[arg(short, long)]
    pub device_id: Option<usize>,

    #[arg(long)]
    pub select_device: bool,
}
