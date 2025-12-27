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
echo "Command: http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5 location=='Wakefield'"
echo
echo "---"
echo

# Test 3: Get total precipitation by month with location filter
echo "3. Testing GET /get_total_precipitation_by_month with location filter"
echo "Command: http GET localhost:3000/get_total_precipitation_by_month month==3 samples==10 location=='Newport Beach'"
http GET localhost:3000/get_total_precipitation_by_month month==3 samples==10 location=='Newport Beach'
echo
echo "---"
echo

# Test 4: Get yearly precipitation
echo "4. Testing GET /get_yearly_precipitation"
echo "Command: http GET localhost:3000/get_yearly_precipitation samples==5 location=='Minneapolis'"
http GET localhost:3000/get_yearly_precipitation samples==5 location=='Minneapolis'
echo
echo "---"
echo

# Additional test cases with different parameters
echo "5. Additional test cases:"
echo

echo "5a. Temperature for different date:"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==1 month==12 samples==3 location=='Pequot Lakes'"
http GET localhost:3000/get_average_temp_by_date day==1 month==12 samples==3 location=='Pequot Lakes'
echo
echo "---"
echo

echo "5b. Precipitation for different month:"
echo "Command: http GET localhost:3000/get_total_precipitation_by_month month==7 samples==5 location=='Altadena'"
http GET localhost:3000/get_total_precipitation_by_month month==7 samples==5 location=='Altadena'
echo
echo "---"
echo

echo "5c. Yearly precipitation for different years:"
echo "Command: http GET localhost:3000/get_yearly_precipitation samples==10 location=='Oakland'"
http GET localhost:3000/get_yearly_precipitation samples==10 location=='Oakland'
echo
echo "---"
echo

# Error cases
echo "6. Testing error cases:"
echo

echo "6a. Invalid month (temperature):"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==15 month==13 samples==5 location=='Athens'"
http GET localhost:3000/get_average_temp_by_date day==15 month==13 samples==5 location=='Athens'
echo
echo "---"
echo

echo "6b. Invalid day:"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==32 month==6 samples==5 location=='Oakland'"
http GET localhost:3000/get_average_temp_by_date day==32 month==6 samples==5 location=='Oakland'
echo
echo "---"
echo

echo "6c. Zero samples (precipitation):"
echo "Command: http GET localhost:3000/get_total_precipitation_by_month month==3 samples==0 location=='Minneapolis'"
http GET localhost:3000/get_total_precipitation_by_month month==3 samples==0 location=='Minneapolis'
echo

echo "6d. Missing location parameter:"
echo "Command: http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5"
http GET localhost:3000/get_average_temp_by_date day==15 month==6 samples==5
echo

echo "6e. Zero years back (yearly precipitation):"
echo "Command: http GET localhost:3000/get_yearly_precipitation samples==0 location=='Boston'"
http GET localhost:3000/get_yearly_precipitation samples==0 location=='Boston'
echo

echo "=== Testing Complete ==="