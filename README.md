# MySQL MCP Gateway

A high-performance MySQL MCP (Model Context Protocol) gateway server built with Rust. It provides a secure HTTP interface for AI assistants to interact with MySQL databases.



## Features

- 🔐 **Token-based Authentication** - Secure access control with Bearer token authentication
- 🚀 **Streamable HTTP Transport** - Implements MCP Streamable HTTP specification
- 🛡️ **Read-only Safety** - Only SELECT queries are allowed for security
- 📊 **Database Exploration** - List databases, tables, and describe table schemas
- 🐳 **Docker Ready** - Easy deployment with Docker



## Available Tools

| Tool | Description | Parameters |
|------|-------------|------------|
| `list_databases` | List all accessible databases | None |
| `list_tables` | List all tables in a database | `database: string` |
| `describe_table` | Show table schema details | `database: string`, `table: string` |
| `query` | Execute SELECT SQL query | `sql: string` |



## Quick Start

### Using Docker (Recommended)

```bash
# Pull and run with config file
docker run -d \
  --name mysql-mcp-gateway \
  -p 5111:5111 \
  -v ~/config.toml:/app/config.toml \
  -v ~/logs:/app/logs \
  -e LOG_DIR=/app/logs \
  ghcr.io/mattyz777/mcp-mysql-gateway:latest
```



## Configuration

Create a `config.toml` file:

```toml
[mysql]
host = "your-mysql-host"
port = 3306
user = "your-username"
password = "your-password"
database = "information_schema"
max_connections = 10

[users]
"username1" = "token1"
"username2" = "token2"
```



### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `LOG_DIR` | Directory for log files | Current directory |



## Usage

### With Claude Code CLI

```bash
claude mcp add my-mysql http://your-server:5111/mcp \
  --transport http \
  --header "Authorization: Bearer your-token"
```



### With curl

```bash
# Initialize connection
curl -X POST http://localhost:5111/mcp \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}'

# List available tools
curl -X POST http://localhost:5111/mcp \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'

# Call a tool
curl -X POST http://localhost:5111/mcp \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"list_databases"}}'
```



