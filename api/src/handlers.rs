use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::{Datelike, Utc};
use serde_json::Value;
use crate::{db::DbPool, models::{Location, TemperatureRequest, TemperatureResponse, PrecipitationRequest, PrecipitationResponse}};

pub async fn get_locations(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Location>>, (StatusCode, String)> {
    let client = pool
        .get()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database connection error".to_string())
        })?;

    let rows = client
        .query("SELECT DISTINCT location FROM daily WHERE location IS NOT NULL ORDER BY location", &[])
        .await
        .map_err(|e| {
            tracing::error!("Failed to query locations: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database query error".to_string())
        })?;

    let locations: Vec<Location> = rows
        .iter()
        .map(|row| Location {
            location: row.get(0),
        })
        .collect();

    Ok(Json(locations))
}

pub async fn get_average_temp_by_date(
    Query(params): Query<TemperatureRequest>,
    State(pool): State<DbPool>,
) -> Result<Json<TemperatureResponse>, (StatusCode, String)> {
    // Validate input parameters
    if params.month == 0 || params.month > 12 {
        return Err((StatusCode::BAD_REQUEST, "Month must be between 1 and 12".to_string()));
    }
    if params.day == 0 || params.day > 31 {
        return Err((StatusCode::BAD_REQUEST, "Day must be between 1 and 31".to_string()));
    }
    if params.samples == 0 {
        return Err((StatusCode::BAD_REQUEST, "Samples must be greater than 0".to_string()));
    }

    let client = pool
        .get()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database connection error".to_string())
        })?;

    // Get current year and calculate target years (current year + 1, +2, etc.)
    let current_year = Utc::now().year();
    let start_year = current_year + 1;
    let end_year = current_year + params.samples as i32;

    // Query for temperature data for the specified day/month across multiple years
    let query = "
        SELECT EXTRACT(YEAR FROM date) as year, data 
        FROM daily 
        WHERE EXTRACT(MONTH FROM date) = $1 
        AND EXTRACT(DAY FROM date) = $2 
        AND EXTRACT(YEAR FROM date) BETWEEN $3 AND $4
        AND location = $5
        AND data IS NOT NULL
        ORDER BY year
    ";

    let rows = client
        .query(query, &[&(params.month as i32), &(params.day as i32), &start_year, &end_year, &params.location])
        .await
        .map_err(|e| {
            tracing::error!("Failed to query temperature data: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database query error".to_string())
        })?;

    if rows.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No temperature data found for the specified date range".to_string()));
    }

    let mut temperatures = Vec::new();
    let mut years_included = Vec::new();

    for row in rows {
        let year: f64 = row.get(0);
        let data_str: String = row.get(1);
        let data: Value = serde_json::from_str(&data_str).unwrap_or_default();
        
        years_included.push(year as i32);

        // Try to get TAVG first, otherwise calculate average of TMIN and TMAX
        if let Some(tavg) = data.get("TAVG") {
            if let Some(temp_val) = tavg.as_f64() {
                temperatures.push(temp_val);
            } else if let Some(temp_str) = tavg.as_str() {
                if let Ok(temp_val) = temp_str.parse::<f64>() {
                    temperatures.push(temp_val);
                }
            }
        } else {
            // Calculate average from TMIN and TMAX
            let tmin = data.get("TMIN").and_then(|v| {
                v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
            });
            let tmax = data.get("TMAX").and_then(|v| {
                v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
            });

            if let (Some(min), Some(max)) = (tmin, tmax) {
                temperatures.push((min + max) / 2.0);
            }
        }
    }

    if temperatures.is_empty() {
        return Err((StatusCode::NOT_FOUND, "No valid temperature data found".to_string()));
    }

    let average_temperature = temperatures.iter().sum::<f64>() / temperatures.len() as f64;

    let response = TemperatureResponse {
        day: params.day,
        month: params.month,
        samples_requested: params.samples,
        samples_found: temperatures.len() as u32,
        average_temperature,
        years_included,
    };

    Ok(Json(response))
}

pub async fn get_total_precipitation_by_month(
    Query(params): Query<PrecipitationRequest>,
    State(pool): State<DbPool>,
) -> Result<Json<PrecipitationResponse>, (StatusCode, String)> {
    // Validate input parameters
    if params.month == 0 || params.month > 12 {
        return Err((StatusCode::BAD_REQUEST, "Month must be between 1 and 12".to_string()));
    }
    if params.samples == 0 {
        return Err((StatusCode::BAD_REQUEST, "Samples must be greater than 0".to_string()));
    }

    let client = pool
        .get()
        .await
        .map_err(|e| {
            tracing::error!("Failed to get database connection: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database connection error".to_string())
        })?;

    // Get current year and calculate target years (current year, current year - 1, etc.)
    let current_year = Utc::now().year();
    let start_year = current_year - params.samples as i32 + 1;
    let end_year = current_year;

    // Query for precipitation data for the specified month across multiple years
    let query = "
        SELECT EXTRACT(YEAR FROM date) as year, data 
        FROM daily 
        WHERE EXTRACT(MONTH FROM date) = $1 
        AND EXTRACT(YEAR FROM date) BETWEEN $2 AND $3
        AND location = $4
        AND data IS NOT NULL
        ORDER BY year
    ";

    let rows = client
        .query(query, &[&(params.month as i32), &start_year, &end_year, &params.location])
        .await
        .map_err(|e| {
            tracing::error!("Failed to query precipitation data: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database query error".to_string())
        })?;

    let mut precipitation_by_year = std::collections::HashMap::new();
    let mut years_with_data = std::collections::HashSet::new();

    for row in rows {
        let year: f64 = row.get(0);
        let year_int = year as i32;
        let data_str: String = row.get(1);
        let data: Value = serde_json::from_str(&data_str).unwrap_or_default();
        
        years_with_data.insert(year_int);

        // Get PRCP value, default to 0.0 if not present
        let prcp_value = data.get("PRCP")
            .and_then(|v| {
                v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
            })
            .unwrap_or(0.0);

        *precipitation_by_year.entry(year_int).or_insert(0.0) += prcp_value;
    }

    let mut years_included: Vec<i32> = years_with_data.into_iter().collect();
    years_included.sort();

    let total_precipitation: f64 = precipitation_by_year.values().sum();

    let response = PrecipitationResponse {
        month: params.month,
        samples_requested: params.samples,
        samples_found: years_included.len() as u32,
        total_precipitation,
        years_included,
    };

    Ok(Json(response))
}
