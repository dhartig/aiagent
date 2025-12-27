# Rust API using Axum

A REST API built with Rust and Axum framework that connects to a PostgreSQL database.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL database running at `sayulita.local`

## Environment Variables

Create a `.env` file in the project root (copy from `.env.example`):

```bash
cp .env.example .env
```

Edit the `.env` file with your database credentials:

```env
# Database Configuration
DB_HOST=sayulita.local
DB_PORT=5432
DB_NAME=mcpdb
DB_USER=your_db_user
DB_PASSWORD=your_db_password

# Server Configuration  
SERVER_PORT=3000
SERVER_HOST=0.0.0.0

# Logging Level
RUST_LOG=api=debug,tower_http=debug
```

## Running the API

```bash
cargo run
```

The server will start on `http://0.0.0.0:3000`

## Endpoints

### GET /get_locations

Returns all distinct locations from the daily table.

**Response:**
```json
[
  {
    "location": "Location 1"
  },
  {
    "location": "Location 2"
  }
]
```

### GET /get_average_temp_by_date

Returns average temperature for a specific day and month across multiple years after the current year.

**Query Parameters:**
- `day` (required): Day of the month (1-31)
- `month` (required): Month (1-12)  
- `samples` (required): Number of years to sample (starting from next year)
- `location` (required): Filter results by specific location

**Example Request:**
```
GET /get_average_temp_by_date?day=15&month=6&samples=5&location=New York
```

**Response:**
```json
{
  "day": 15,
  "month": 6,
  "samples_requested": 5,
  "samples_found": 3,
  "average_temperature": 25.4,
  "years_included": [2026, 2027, 2028]
}
```

**Temperature Calculation:**
- Uses `TAVG` from the JSONB data field if available
- If `TAVG` is not present, calculates average of `TMIN` and `TMAX`

### GET /get_total_precipitation_by_month

Returns total precipitation for a specific month across multiple years going back from the current year.

**Query Parameters:**
- `month` (required): Month (1-12)  
- `samples` (required): Number of years to sample (going back from current year)
- `location` (required): Filter results by specific location

**Example Request:**
```
GET /get_total_precipitation_by_month?month=3&samples=10&location=Los Angeles
```

**Response:**
```json
{
  "month": 3,
  "samples_requested": 10,
  "samples_found": 8,
  "total_precipitation": 145.6,
  "years_included": [2016, 2017, 2018, 2020, 2021, 2022, 2024, 2025]
}
```

**Precipitation Calculation:**
- Sums all `PRCP` values from the JSONB data field for the specified month
- If `PRCP` field is not available for a record, uses 0 as the value
- Aggregates by year and then sums across all years

### GET /get_yearly_precipitation

Returns total yearly precipitation going back from the current year for a specific location.

**Query Parameters:**
- `samples` (required): Number of years to go back from current year
- `location` (required): Filter results by specific location

**Example Request:**
```
GET /get_yearly_precipitation?samples=5&location=Los Angeles
```

**Response:**
```json
{
  "samples": 5,
  "samples_found": 4,
  "yearly_precipitation": {
    "2021": 245.6,
    "2022": 189.3,
    "2024": 312.8,
    "2025": 156.4
  }
}
```

**Yearly Precipitation Calculation:**
- Sums all daily `PRCP` values for each entire year
- Aggregates across all months and days in the year
- Returns total precipitation by year as a key-value map

## Building for Production

```bash
cargo build --release
```

The compiled binary will be in `target/release/api`
