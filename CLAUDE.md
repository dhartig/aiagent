# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an MCP (Model Context Protocol) demo project with four loosely-coupled components:
- `api/` — Rust/Axum REST API for querying historical weather data from PostgreSQL
- `weather/` — Rust MCP server exposing NWS weather tools via stdio transport
- `mcp-client-python/` — Python MCP client that connects Claude to MCP servers
- `dataprep/` — Jupyter notebooks for fetching/storing historical weather data
- `infrastructure/` — microk8s Kubernetes manifests for PostgreSQL deployment

## Commands

### API (`api/`)
```bash
cargo build --release
cargo run
./test_endpoints.sh   # integration test against running server
```
Requires `.env` (copy from `.env.example`) with PostgreSQL credentials.

### Weather MCP Server (`weather/`)
```bash
cargo build
cargo run   # communicates via stdio (MCP transport)
```

### MCP Python Client (`mcp-client-python/`)
```bash
uv sync
python client.py ./weather/target/debug/weather   # pass path to MCP server binary
```
Requires `.env` with `ANTHROPIC_API_KEY`.

### Data Prep (`dataprep/`)
```bash
poetry install   # or pip install via pyproject.toml
jupyter lab
```

### Infrastructure
```bash
cd infrastructure && ./deploy.sh
```

## Architecture

### Data Flow
```
User → MCP Python Client (Claude + tool calling)
         ├── Weather MCP Server (NWS live alerts/forecasts via stdio)
         └── REST API (Axum) → PostgreSQL (historical daily weather data)
```

### API Design (`api/`)
Four GET endpoints with query parameters (`day`, `month`, `samples`, `location`):
- `GET /get_locations`
- `GET /get_average_temp_by_date`
- `GET /get_total_precipitation_by_month`
- `GET /get_yearly_precipitation`

Key files: `main.rs` (Axum router + CORS + tracing), `db.rs` (deadpool-postgres pool), `handlers.rs` (business logic + JSONB extraction), `models.rs` (serde structs).

Weather observations are stored as JSONB with keys `TAVG`, `TMIN`, `TMAX`, `PRCP`. Queries use PostgreSQL `EXTRACT()` for date filtering.

### Weather MCP Server (`weather/`)
Uses the `rmcp` crate with macro-based tool routing. Exposes two tools (`get_alerts(state)`, `get_forecast(lat, lon)`) via HTTP to `api.weather.gov`. Stdio transport means the server is launched as a subprocess by the client.

### MCP Python Client (`mcp-client-python/`)
`MCPClient` class manages the full agentic loop: connect to server → list tools → send to Claude → handle `tool_use` responses by calling `session.call_tool()` → feed results back to Claude → repeat until final response. Uses `claude-sonnet-4-5` model.

## Database

PostgreSQL at `sayulita.local:5432`, database `aiagent_db`. The daily table uses JSONB for flexible weather data. Schema creation script: `dataprep/make_daily_table.sql`. Infrastructure deploys Postgres 15 on microk8s with NodePort 30432 and `postgres.local` ingress.
