#!/bin/bash

# NASA CLI Test Script
# Tests all CLI commands to ensure they're working correctly

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# CLI binary path
NASA_CLI="${1:-./target/release/nasa-cli}"

# Check if CLI exists
if [ ! -f "$NASA_CLI" ]; then
    echo -e "${RED}Error: NASA CLI not found at $NASA_CLI${NC}"
    echo "Please build the CLI first with: cargo build --release --features cli"
    exit 1
fi

echo "Testing NASA CLI at: $NASA_CLI"
echo "========================================"

# Test function
test_command() {
    local name="$1"
    local command="$2"
    local expected_exit_code="${3:-0}"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    echo -n "Testing $name... "
    
    # Run command and capture exit code
    output=$($NASA_CLI $command 2>&1)
    exit_code=$?
    
    if [ "$exit_code" = "$expected_exit_code" ]; then
        echo -e "${GREEN}✓ OK${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ FAILED (exit code: $exit_code, expected: $expected_exit_code)${NC}"
        echo "  Command: $NASA_CLI $command"
        echo "  Output: $output" | head -n 5
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Test configuration commands
echo -e "\n--- Configuration Commands ---"
test_command "Config show" "config show"
test_command "Config init" "config init"
test_command "Config set API endpoint" "config set api_endpoint https://nasa-api.workers.dev"
test_command "Config set output format" "config set output_format json"

# Test cache commands
echo -e "\n--- Cache Commands ---"
test_command "Cache stats" "cache stats"
# Don't clear cache during tests to preserve cached responses
# test_command "Cache clear" "cache clear"

# Test APOD commands
echo -e "\n--- APOD Commands ---"
test_command "APOD today" "apod today -o json"
test_command "APOD specific date" "apod date 2023-01-01 -o json"
test_command "APOD random" "apod random --count 2 -o json"

# Test NEO/Asteroids commands
echo -e "\n--- Asteroids Commands ---"
test_command "Asteroids feed (default dates)" "asteroids feed -o json"
test_command "Asteroids feed (date range)" "asteroids feed --start-date 2024-01-01 --end-date 2024-01-07 -o json"
test_command "Asteroids lookup" "asteroids lookup 3542519 -o json"
test_command "Asteroids browse" "asteroids browse --page 1 --size 5 -o json"

# Test DONKI commands
echo -e "\n--- DONKI (Space Weather) Commands ---"
test_command "DONKI CME" "donki cme --start-date 2023-01-01 -o json"
test_command "DONKI solar flares" "donki flare --start-date 2023-01-01 -o json"
test_command "DONKI geomagnetic storms" "donki storm -o json"
test_command "DONKI notifications" "donki notifications -o json"

# Test Mars Rover commands
echo -e "\n--- Mars Rover Commands ---"
test_command "Mars photos by sol" "mars photos curiosity --sol 1000 -o json"
test_command "Mars photos with camera" "mars photos curiosity --sol 1000 --camera NAVCAM -o json"
test_command "Mars latest photos" "mars latest curiosity -o json"
test_command "Mars mission manifest" "mars manifest curiosity -o json"

# Test Earth imagery commands
echo -e "\n--- Earth Imagery Commands ---"
test_command "Earth assets" "earth assets --lat 29.78 --lon -95.33 --date 2023-01-01 -o json"
# Earth image command downloads binary data, so we just test if it runs
test_command "Earth image (dry run)" "earth image --lat 29.78 --lon -95.33 --date 2023-01-01 -o json" 2

# Test EPIC commands
echo -e "\n--- EPIC Commands ---"
test_command "EPIC natural all dates" "epic natural all -o json"
test_command "EPIC natural specific date" "epic natural 2023-01-01 -o json"
test_command "EPIC enhanced all dates" "epic enhanced all -o json"

# Test Tech Transfer commands
echo -e "\n--- Tech Transfer Commands ---"
test_command "Tech patents search" "tech patents --query propulsion -o json"
test_command "Tech software" "tech software -o json"
test_command "Tech spinoffs" "tech spinoffs -o json"

# Test Media commands
echo -e "\n--- Media Library Commands ---"
test_command "Media search basic" "media search --query apollo -o json"
test_command "Media search advanced" "media search --query mars --media-type image --center JPL -o json"
# Skip asset commands as they require valid NASA IDs
# test_command "Media asset" "media asset NHQ202301230001 -o json"

# Test Exoplanets commands
echo -e "\n--- Exoplanets Commands ---"
test_command "Exoplanets custom query" "exoplanets search \"select * from ps limit 5\" -o json"
test_command "Exoplanets Kepler search" "exoplanets kepler -o json"
test_command "Exoplanets Kepler with name" "exoplanets kepler --name 452 -o json"

# Test SSD/CNEOS commands
echo -e "\n--- SSD/CNEOS Commands ---"
test_command "SSD close approach" "ssd close-approach --date-min 2024-01-01 --date-max 2024-01-07 -o json"
test_command "SSD close approach PHA only" "ssd close-approach --pha -o json"
test_command "SSD fireballs" "ssd fireballs --date-min 2023-01-01 -o json"
test_command "SSD sentry" "ssd sentry -o json"

# Test output formats
echo -e "\n--- Output Format Tests ---"
test_command "Output JSON" "apod today -o json"
test_command "Output table" "asteroids browse --size 5 -o table"
test_command "Output CSV" "ssd fireballs --date-min 2023-01-01 -o csv"
test_command "Output pretty (default)" "apod today -o pretty"

# Test global options
echo -e "\n--- Global Options Tests ---"
test_command "No cache option" "apod today --no-cache -o json"
test_command "Custom endpoint" "apod today --endpoint http://localhost:8787 -o json" 1

# Test help commands
echo -e "\n--- Help Commands ---"
test_command "Main help" "--help"
test_command "APOD help" "apod --help"
test_command "Mars help" "mars --help"
test_command "Config help" "config --help"

# Summary
echo -e "\n========================================"
echo "Test Summary:"
echo -e "Tests run: $TESTS_RUN"
echo -e "Tests passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}Some tests failed!${NC}"
    exit 1
fi