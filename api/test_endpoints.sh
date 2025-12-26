#!/bin/bash

# Test script for API endpoints using HTTPie
# Make sure the API server is running on localhost:3000 before running these commands

echo "=== Testing API Endpoints ==="
echo

# Test 1: Get all locations
echo "1. Testing GET /get_locations"
echo "Command: http GET localhost:3000/get_locations"
http GET localhost:3000/get_locations
echo
echo "---"
echo

# Test 2: Get average temperature by date with location filter
echo "2. Testing GET /get_average_temp_by_date with location filter"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5 location=='New York'"
http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5 location=='New York'
echo
echo "---"
echo

# Test 3: Get total precipitation by month with location filter
echo "3. Testing GET /get_total_precipitation_by_month with location filter"
echo "Command: http GET localhost:3000/get_total_precipitation_by_month month==3 samples==10 location=='Los Angeles'"
http GET localhost:3000/get_total_precipitation_by_month month==3 samples==10 location=='Los Angeles'
echo
echo "---"
echo

# Additional test cases with different parameters
echo "4. Additional test cases:"
echo

echo "4a. Temperature for different date:"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==1 month==12 samples==3 location=='Chicago'"
http GET localhost:3000/get_average_temp_by_date day==1 month==12 samples==3 location=='Chicago'
echo
echo "---"
echo

echo "4b. Precipitation for different month:"
echo "Command: http GET localhost:3000/get_total_precipitation_by_month month==7 samples==5 location=='Miami'"
http GET localhost:3000/get_total_precipitation_by_month month==7 samples==5 location=='Miami'
echo
echo "---"
echo

# Error cases
echo "5. Testing error cases:"
echo

echo "5a. Invalid month (temperature):"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==15 month==13 samples==5 location=='Boston'"
http GET localhost:3000/get_average_temp_by_date day==15 month==13 samples==5 location=='Boston'
echo
echo "---"
echo

echo "5b. Invalid day:"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==32 month==6 samples==5 location=='Seattle'"
http GET localhost:3000/get_average_temp_by_date day==32 month==6 samples==5 location=='Seattle'
echo
echo "---"
echo

echo "5c. Zero samples (precipitation):"
echo "Command: http GET localhost:3000/get_total_precipitation_by_month month==3 samples==0 location=='Denver'"
http GET localhost:3000/get_total_precipitation_by_month month==3 samples==0 location=='Denver'
echo

echo "5d. Missing location parameter:"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5"
http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5
echo

echo "=== Testing Complete ==="