# API Endpoint Testing Commands

This file contains HTTPie commands to test all the API endpoints.

## Prerequisites
- Install HTTPie: `pip install httpie` or `sudo apt install httpie`
- Start the API server: `cargo run`
- Server should be running on `localhost:3000`

## Basic Commands

### 1. Get All Locations
```bash
http GET localhost:3000/get_locations
```

### 2. Get Average Temperature by Date
```bash
# Get average temperature for June 15th across 5 years after current year
http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5 location=='New York'

# Get average temperature for December 1st across 3 years after current year
http GET localhost:3000/get_average_temp_by_date day==1 month==12 samples==3 location=='Chicago'

# Get average temperature for March 20th
http GET localhost:3000/get_average_temp_by_date day==20 month==3 samples==2 location=='Boston'
```

### 3. Get Total Precipitation by Month
```bash
# Get total precipitation for March across 10 years back from current year
http GET localhost:3000/get_total_precipitation_by_month month==3 samples==10 location=='Los Angeles'

# Get total precipitation for July across 5 years back from current year
http GET localhost:3000/get_total_precipitation_by_month month==7 samples==5 location=='Miami'

# Get total precipitation for December
http GET localhost:3000/get_total_precipitation_by_month month==12 samples==15 location=='Seattle'
```

### 4. Get Yearly Precipitation
```bash
# Get yearly precipitation totals for the last 5 years
http GET localhost:3000/get_yearly_precipitation samples==5 location=='Chicago'

# Get yearly precipitation for the last 10 years
http GET localhost:3000/get_yearly_precipitation samples==10 location=='Denver'

# Get yearly precipitation for the last 3 years
http GET localhost:3000/get_yearly_precipitation samples==3 location=='Boston'
```

## Error Test Cases

### Invalid Parameters
```bash
# Invalid month (should return 400 Bad Request)
http GET localhost:3000/get_average_temp_by_date day==15 month==13 samples==5 location=='Boston'

# Invalid day (should return 400 Bad Request)
http GET localhost:3000/get_average_temp_by_date day==32 month==6 samples==5 location=='Seattle'

# Zero samples (should return 400 Bad Request)
http GET localhost:3000/get_total_precipitation_by_month month==3 samples==0 location=='Denver'

# Zero years back (should return 400 Bad Request)
http GET localhost:3000/get_yearly_precipitation samples==0 location=='Miami'

# Missing location parameter (should return 400 Bad Request)
http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5
```

## Using curl instead of HTTPie

If you prefer curl, here are equivalent commands:

```bash
# Get locations
curl "http://localhost:3000/get_locations"

# Get temperature (location now required)
curl "http://localhost:3000/get_average_temp_by_date?day=15&month=6&samples=5&location=New%20York"

# Get precipitation (location now required)
curl "http://localhost:3000/get_total_precipitation_by_month?month=3&samples=10&location=Los%20Angeles"

# Get yearly precipitation (location required)
curl "http://localhost:3000/get_yearly_precipitation?samples=5&location=Chicago"
```

## Expected Response Formats

### Locations Response
```json
[
  {
    "location": "Station A"
  },
  {
    "location": "Station B"
  }
]
```

### Temperature Response
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

### Precipitation Response
```json
{
  "month": 3,
  "samples_requested": 10,
  "samples_found": 8,
  "precipitation_by_year": {
    "2016": 45.6,
    "2017": 23.4,
    "2018": 67.2
  }
}
```

### Yearly Precipitation Response
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

## Running the Test Script

Make the test script executable and run it:

```bash
chmod +x test_endpoints.sh
./test_endpoints.sh
```

This will run all test cases automatically and display the results.