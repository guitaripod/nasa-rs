#!/bin/bash

# NASA API Worker Test Script
# Tests all endpoints to ensure they're working correctly

BASE_URL="${1:-http://localhost:8787}"

echo "Testing NASA API Worker at: $BASE_URL"
echo "========================================"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Test function
test_endpoint() {
    local name="$1"
    local endpoint="$2"
    local params="$3"
    
    echo -n "Testing $name... "
    
    if [ -n "$params" ]; then
        url="$BASE_URL$endpoint?$params"
    else
        url="$BASE_URL$endpoint"
    fi
    
    response=$(curl -s -o /dev/null -w "%{http_code}" "$url")
    
    if [ "$response" = "200" ]; then
        echo -e "${GREEN}✓ OK${NC}"
    else
        echo -e "${RED}✗ FAILED (HTTP $response)${NC}"
        # Show actual response for debugging
        echo "  URL: $url"
        curl -s "$url" | head -n 5
        echo ""
    fi
}

# Test health endpoint
test_endpoint "Health Check" "/health" ""

# Test APOD endpoints
echo -e "\n--- APOD ---"
test_endpoint "APOD Today" "/api/apod" ""
test_endpoint "APOD Specific Date" "/api/apod" "date=2023-01-01"
test_endpoint "APOD Random" "/api/apod" "count=1"

# Test NEO endpoints
echo -e "\n--- Near Earth Objects ---"
test_endpoint "NEO Feed" "/api/neo/feed" ""
test_endpoint "NEO Lookup" "/api/neo/3542519" ""
test_endpoint "NEO Browse" "/api/neo/browse" "page=0&size=20"

# Test DONKI endpoints
echo -e "\n--- DONKI (Space Weather) ---"
test_endpoint "CME Events" "/api/donki/cme" ""
test_endpoint "Solar Flares" "/api/donki/flr" ""
test_endpoint "Geomagnetic Storms" "/api/donki/gst" ""
test_endpoint "Notifications" "/api/donki/notifications" ""

# Test Mars Rover endpoints
echo -e "\n--- Mars Rover Photos ---"
test_endpoint "Curiosity Photos" "/api/mars-photos/curiosity/photos" "sol=1000"
test_endpoint "Curiosity Latest" "/api/mars-photos/curiosity/latest" ""
test_endpoint "Curiosity Manifest" "/api/mars-photos/manifests/curiosity" ""

# Test Earth imagery endpoints
echo -e "\n--- Earth Imagery ---"
test_endpoint "Earth Assets" "/api/earth/assets" "lat=29.78&lon=-95.33&date=2023-01-01"
# Note: Earth imagery endpoint returns binary data, so we skip it in basic tests

# Test EPIC endpoints
echo -e "\n--- EPIC ---"
test_endpoint "EPIC Natural All" "/api/epic/natural/all" ""
test_endpoint "EPIC Natural Date" "/api/epic/natural/date/2023-01-01" ""
test_endpoint "EPIC Enhanced All" "/api/epic/enhanced/all" ""

# Test Tech Transfer endpoints
echo -e "\n--- Tech Transfer ---"
test_endpoint "Patents" "/api/techtransfer/patents" "query=propulsion"
test_endpoint "Software" "/api/techtransfer/software" ""
test_endpoint "Spinoffs" "/api/techtransfer/spinoffs" ""

# Test Media endpoints
echo -e "\n--- NASA Image and Video Library ---"
test_endpoint "Media Search" "/api/media/search" "q=apollo"
test_endpoint "Media Asset" "/api/media/asset/NHQ202301230001" ""
test_endpoint "Media Metadata" "/api/media/metadata/NHQ202301230001" ""

# Test Exoplanets endpoint
echo -e "\n--- Exoplanets ---"
test_endpoint "Exoplanets Query" "/api/exoplanets/query" "query=select+*+from+ps+limit+5&format=json"

# Test SSD/CNEOS endpoints
echo -e "\n--- SSD/CNEOS ---"
test_endpoint "Close Approach Data" "/api/ssd/cad" "date-min=2024-01-01&date-max=2024-01-07"
test_endpoint "Small Body Database" "/api/ssd/sbdb" "sstr=433"
test_endpoint "Sentry" "/api/ssd/sentry" ""
test_endpoint "Fireballs" "/api/ssd/fireballs" ""
test_endpoint "NHATS" "/api/ssd/nhats" ""

echo -e "\n========================================"
echo "Test complete!"