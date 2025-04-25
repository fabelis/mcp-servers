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

            let protocol = get_server_protocol()?;

            let mut port = env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or(3000);

            // Keep trying ports until we find an available one
            while std::net::TcpListener::bind(format!("0.0.0.0:{}", port)).is_err() {
                if port == u16::MAX {
                    anyhow::bail!("No available ports found");
                }
                tracing::warn!("Port {} is already in use, trying {}", port, port + 1);
                port += 1;
            }

            Server::start(ServerSseTransport::new(
                "0.0.0.0".to_string(),
                port,
                protocol,
            ))
            .await
        }
        TransportType::Stdio => {
            // Prevents the server from logging to stdout
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .with_writer(std::io::stderr)
                .init();

            let protocol = get_server_protocol()?;

            Server::start(ServerStdioTransport::new(protocol)).await
        }
    }
}

fn get_server_protocol() -> Result<Protocol> {
    if cfg!(feature = "arvix") {
        tracing::info!("Starting Arvix server");
        #[cfg(feature = "arvix")]
        return Ok(servers::arvix::server::protocol());
    } else if cfg!(feature = "twitter") {
        tracing::info!("Starting Twitter server");
        #[cfg(feature = "twitter")]
        return Ok(servers::twitter::server::protocol());
    } else if cfg!(feature = "discord") {
        tracing::info!("Starting Discord server");
        #[cfg(feature = "discord")]
        return Ok(servers::discord::server::protocol());
    } else if cfg!(feature = "shopify") {
        tracing::info!("Starting Shopify server");
        #[cfg(feature = "shopify")]
        return Ok(servers::shopify::server::protocol());
    } else if cfg!(feature = "huggingface") {
        tracing::info!("Starting HuggingFace server");
        #[cfg(feature = "huggingface")]
        return Ok(servers::huggingface::server::protocol());
    } else if cfg!(feature = "replicate") {
        tracing::info!("Starting Replicate server");
        #[cfg(feature = "replicate")]
        return Ok(servers::replicate::server::protocol());
    } else {
        anyhow::bail!("No server selected");
    }
}
