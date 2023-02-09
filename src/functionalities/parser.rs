use clap::{Args, Parser, Subcommand};
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
struct SmartHandler {
    address: Option<String>,

    /// Optional port incase the user wants to change it
    #[arg(short, long)]
    port: Option<String>,

    /// Optional device id incase the user wants to change it
    #[arg(short, long)]
    device_id: Option<String>,
}

pub fn parse_args() -> ParsedArgs {
    let cli = ParsedArgs::parse();

    match &cli.command {
        Commands::Client(args) => {
            println!("running Client mode\n");
            println!("params: \n\t address: {:?}\n\t port: {:?}\n\t deviceID: {:?}", args.address, args.port, args.device_id);

        }
        Commands::Server(args) => {
            println!("running Server mode\n");
            println!("params: \n\t address: {:?}\n\t port: {:?}\n\t deviceID: {:?}", args.address, args.port, args.device_id);
        }
    }

    return cli;
}