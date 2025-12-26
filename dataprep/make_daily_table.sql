CREATE DATABASE mcpdb;

\c mcpdb;

CREATE TABLE daily (
    id SERIAL PRIMARY KEY,
    location TEXT,
    station_id TEXT,
    station_name TEXT,
    date DATE,
    data JSONB
);

