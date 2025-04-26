<div align="center">

# Fabelis MCP Servers

A collection of composable MCP servers built in Rust

<img src="assets/banner.png" alt="Project Banner" width="100%" />

Fabelis MCP Servers is a powerful collection of composable servers built in Rust using our [mcp-core]() framework. This project enables you to easily spin up and manage your own local MCP servers, providing seamless integration with various platforms and services.

[![License](https://img.shields.io/badge/License-MIT-blue)](./LICENSE)
[![Discord](https://img.shields.io/badge/Discord-Join%20Us-blue)](https://discord.gg/your-invite)
[![GitHub Stars](https://img.shields.io/github/stars/fabelis/mcp-servers)](https://github.com/fabelis/mcp-servers/stargazers)
[![Twitter Follow](https://img.shields.io/twitter/follow/FabelisAI?style=social)](https://x.com/FabelisAI)
[![Website](https://img.shields.io/badge/Website-fabelis.ai-blue)](https://fabelis.ai/)

</div>

## Supported Platforms

<div align="center" style="display: flex; flex-wrap: wrap; gap: 20px; justify-content: center;">

<div align="center">
  <img src="assets/server_logos/discord.png" alt="Discord Logo" width="100" />
  <p><strong>Discord</strong></p>
</div>

<div align="center">
  <img src="assets/server_logos/Shopify.com_Symbol_1.png" alt="Shopify Logo" width="100" />
  <p><strong>Shopify</strong></p>
</div>

<div align="center">
  <img src="assets/server_logos/x.png" alt="X (Twitter) Logo" width="100" />
  <p><strong>X (Twitter)</strong></p>
</div>

<div align="center">
  <img src="assets/server_logos/arxiv.png" alt="arXiv Logo" width="100" />
  <p><strong>arXiv</strong></p>
</div>

<div align="center">
  <img src="assets/server_logos/huggingface.png" alt="Hugging Face Logo" width="100" />
  <p><strong>Hugging Face</strong></p>
</div>

<div align="center">
  <img src="assets/server_logos/replicate.png" alt="Replicate Logo" width="100" />
  <p><strong>Replicate</strong></p>
</div>

</div>

## Table of Contents

- [Fabelis MCP Servers](#fabelis-mcp-servers)
  - [Supported Platforms](#supported-platforms)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
  - [Usage](#usage)
    - [Environment Setup](#environment-setup)
    - [Running Servers](#running-servers)
    - [SSE Server Notes](#sse-server-notes)
  - [Contributing](#contributing)
  - [License](#license)

## Getting Started

### Prerequisites

- Rust toolchain installed
- Cargo package manager
- Git

### Installation

1. Clone the repository
```bash
git clone https://github.com/fabelis/mcp-servers.git
cd mcp-servers
```

## Usage

### Environment Setup

1. Create your environment file:
```bash
cp .env.example .env
```

2. Configure your environment variables:
```env
# Shopify Server
SHOPIFY_SHOP_DOMAIN="FILL_WITH_YOUR_DOMAIN"
SHOPIFY_ACCESS_TOKEN="FILL_WITH_YOUR_ACCESS_TOKEN"
```

### Running Servers

To run a specific server:
```bash
cargo run --server arxiv
```

For SSE server mode:
```bash
cargo run sse --server arxiv
```

To run with specific features:
```bash
cargo run --no-default-features --features arxiv sse --server arxiv
```

### SSE Server Notes

The server will run on port `3000` by default, but you can specify a custom port in your `.env` file using `SERVER_PORT=""`. If the selected port is in use, it will automatically try the next available port.

Connect to your SSE server at: `http://localhost:{YOUR_PORT}/sse`

## Contributing

We welcome contributions! If you'd like to add your own MCP tools created using [mcp-core](), please follow our contribution guidelines and open a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.