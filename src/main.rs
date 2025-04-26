mod servers;
use std::env;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use mcp_core::{
    server::Server,
    transport::{ServerSseTransport, ServerStdioTransport},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Transport type to use
    #[arg(value_enum, default_value_t = TransportType::Stdio)]
    transport: TransportType,

    /// Which server to run
    #[arg(value_enum, short, long)]
    server: ServerType,

    /// Optional path to .env file
    #[arg(short, long)]
    env_file: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum TransportType {
    Sse,
    Stdio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ServerType {
    Arxiv,
    Twitter,
    Discord,
    Shopify,
    HuggingFace,
    Replicate,
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

    // Select the server based on the CLI argument
    let protocol = match cli.server {
        ServerType::Arxiv => {
            #[cfg(feature = "arxiv")]
            {
                servers::arxiv::server::protocol()
            }
            #[cfg(not(feature = "arxiv"))]
            {
                anyhow::bail!("arxiv feature is not enabled")
            }
        }
        ServerType::Twitter => {
            #[cfg(feature = "twitter")]
            {
                servers::twitter::server::protocol()
            }
            #[cfg(not(feature = "twitter"))]
            {
                anyhow::bail!("Twitter feature is not enabled")
            }
        }
        ServerType::Discord => {
            #[cfg(feature = "discord")]
            {
                servers::discord::server::protocol()
            }
            #[cfg(not(feature = "discord"))]
            {
                anyhow::bail!("Discord feature is not enabled")
            }
        }
        ServerType::Shopify => {
            #[cfg(feature = "shopify")]
            {
                servers::shopify::server::protocol()
            }
            #[cfg(not(feature = "shopify"))]
            {
                anyhow::bail!("Shopify feature is not enabled")
            }
        }
        ServerType::HuggingFace => {
            #[cfg(feature = "huggingface")]
            {
                servers::huggingface::server::protocol()
            }
            #[cfg(not(feature = "huggingface"))]
            {
                anyhow::bail!("HuggingFace feature is not enabled")
            }
        }
        ServerType::Replicate => {
            #[cfg(feature = "replicate")]
            {
                servers::replicate::server::protocol()
            }
            #[cfg(not(feature = "replicate"))]
            {
                anyhow::bail!("Replicate feature is not enabled")
            }
        }
    };

    match cli.transport {
        TransportType::Sse => {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .init();

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
            tracing::info!("Starting server on port {}", port);

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

            Server::start(ServerStdioTransport::new(protocol)).await
        }
    }
}
