# Weather MCP Server

A Rust MCP server that exposes live weather data from the [National Weather Service API](https://www.weather.gov/documentation/services-web-api). Supports both stdio (subprocess) and remote HTTP transports.

## Tools

- `get_alerts(state)` — Active weather alerts for a US state (e.g. `"CA"`)
- `get_forecast(latitude, longitude)` — 5-period forecast for a coordinate pair

## Building

```bash
cargo build --release
```

## Usage

### Remote HTTP server (default)

Starts an HTTP server using the MCP Streamable HTTP transport. The MCP endpoint is at `/mcp`.

```bash
cargo run --release
```

The server binds to `0.0.0.0:8080` by default. Override the port with the `PORT` environment variable:

```bash
PORT=3000 cargo run --release
```

Clients should connect to `http://<host>:<port>/mcp`.

Shut down with Ctrl+C.

### Stdio (subprocess) transport

Pass `--stdio` to use stdin/stdout instead — this is the original mode used when the server is launched as a subprocess by an MCP client.

```bash
cargo run --release -- --stdio
```
