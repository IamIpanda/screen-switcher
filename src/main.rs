mod switcher;
mod server;
use std::net::SocketAddr;

use clap::{Parser, Subcommand};
use switcher::Connection;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Matrix position
    #[arg(short, long, env="SCREEN_SWTICHER_TARGET")]
    target: SocketAddr,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set screen link
    Set {
        from: u8,
        to: u8
    },
    /// Make it 1->1, 2->2, etc.
    Reset,
    /// Save to slot [pos]
    Save {
        pos: Option<u8>
    },
    /// Load from slot [pos]
    Load {
        pos: Option<u8>
    },
    /// Start a http server to operate.
    Serve {
        #[arg(short, long)]
        port: Option<u16>
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cli = Cli::parse();
    let connection = Connection::new(cli.target);
    
    match cli.command {
        Commands::Save { pos } => connection.save(pos.unwrap_or(1)),
        Commands::Load { pos } => connection.load(pos.unwrap_or(1)),
        Commands::Set { from, to } => connection.set(from, to),
        Commands::Reset => connection.reset(),
        Commands::Serve{ port } => server::start_server(connection, port.unwrap_or(8080)).await
    };
}
