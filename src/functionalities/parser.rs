use clap::{Args, Parser, Subcommand};
use crate::functionalities::client::ClientHandler;
use crate::functionalities::server::ServerHandler;
// use crate::audio_utils::DeviceManager;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct ParsedArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    // Client(Test),

    /// Run the server
    Server(SmartHandler),
     
    /// Run the client
    Client(SmartHandler),
}

#[derive(Args)]
pub struct SmartHandler {
    pub address: Option<String>,

    /// Optional port incase the user wants to change it
    #[arg(short, long, default_value = "55452")]
    pub port: Option<String>,

    /// Optional device id incase the user wants to change it
    #[arg(short, long, default_value = "0")]
    pub device_id: Option<String>,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub select_device: bool,
}

pub fn parse_args() -> ParsedArgs {
    let cli = ParsedArgs::parse();
    

    match &cli.command {
        Commands::Client(args) => {
            let client_handler =  ClientHandler::new(args);
            client_handler.run();
        }
        Commands::Server(args) => {
            let server_handler = ServerHandler::new(args);
            server_handler.run();
        }
    }

    return cli;
}