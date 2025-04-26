<div align="center">

# Fabelis MCP Servers

A collection of composable MCP servers built in Rust

<img src="assets/banner.png" alt="Project Banner" width="100%" />

Fabelis MCP Servers is a powerful collection of composable servers built in Rust using our [mcp-core]() framework. This project enables you to easily spin up and manage your own local MCP servers, providing seamless integration with various platforms and services.

[![License](https://img.shields.io/badge/License-MIT-blue)](./LICENSE)
[![GitHub Repo stars](https://img.shields.io/github/stars/fabelis/mcp-servers)](https://github.com/fabelis/mcp-servers)
[![Twitter Follow](https://img.shields.io/twitter/follow/FabelisAI?style=social)](https://x.com/FabelisAI)
[![Website](https://img.shields.io/badge/Website-fabelis.ai-blue)](https://fabelis.ai/)

</div>

## Supported Platforms

<table>
  <tr>
    <td align="center">
      <a href="src/servers/discord">
        <img src="assets/server_logos/discord.png" width="100" alt="Discord" /><br/>
        <strong>Discord</strong>
      </a>
    </td>
    <td align="center">
      <a href="src/servers/shopify">
        <img src="assets/server_logos/Shopify.com_Symbol_1.png" width="100" alt="Shopify" /><br/>
        <strong>Shopify</strong>
      </a>
    </td>
    <td align="center">
      <a href="src/servers/twitter">
        <img src="assets/server_logos/x.png" width="100" alt="X (Twitter)" /><br/>
        <strong>X (Twitter)</strong>
      </a>
    </td>
    <td align="center">
      <a href="src/servers/arxiv">
        <img src="assets/server_logos/arxiv.png" width="100" alt="arXiv" /><br/>
        <strong>arXiv</strong>
      </a>
    </td>
    <td align="center">
      <a href="src/servers/huggingface">
        <img src="assets/server_logos/huggingface.png" width="100" alt="Hugging Face" /><br/>
        <strong>Hugging Face</strong>
      </a>
    </td>
    <td align="center">
      <a href="src/servers/replicate">
        <img src="assets/server_logos/replicate.png" width="100" alt="Replicate" /><br/>
        <strong>Replicate</strong>
      </a>
    </td>
  </tr>
</table>

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