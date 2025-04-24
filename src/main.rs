mod servers;
use std::env;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use mcp_core::{
    protocol::Protocol,
    server::Server,
    transport::{ServerSseTransport, ServerStdioTransport},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Transport type to use
    #[arg(value_enum, default_value_t = TransportType::Stdio)]
    transport: TransportType,

    /// Optional path to .env file
    #[arg(short, long)]
    env_file: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum TransportType {
    Sse,
    Stdio,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load env file from path if provided, otherwise load from default location
    if let Some(env_path) = cli.env_file {
        dotenv::from_path(env_path).ok();
    } else {
        dotenv().ok();
    }

    match cli.transport {
        TransportType::Sse => {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .init();

            Server::start(ServerSseTransport::new(
                "0.0.0.0".to_string(),
                env::var("SERVER_PORT")
                    .ok()
                    .and_then(|p| p.parse::<u16>().ok())
                    .unwrap_or(3000),
                get_server_protocol(),
            ))
            .await
        }
        TransportType::Stdio => {
            // Prevents the server from logging to stdout
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .with_writer(std::io::stderr)
                .init();

            Server::start(ServerStdioTransport::new(get_server_protocol())).await
        }
    }
}

fn get_server_protocol() -> Protocol {
    #[cfg(feature = "arvix")]
    return servers::arvix::server::protocol();
}
