# NASA-RS Testing Guide

This guide covers all testing aspects of the NASA-RS project, including unit tests, integration tests, and manual testing procedures.

## Quick Start

Run all tests:
```bash
make test
```

## Test Categories

### 1. Unit Tests

Run unit tests for both worker and CLI:
```bash
cargo test --all-features
```

Run tests for specific module:
```bash
cargo test models::
cargo test handlers::
cargo test cli::
```

Run tests with output displayed:
```bash
cargo test --all-features -- --nocapture
```

### 2. Worker Endpoint Tests

Test all worker endpoints using the provided script:
```bash
./scripts/test_endpoints.sh

# Test against local development server
./scripts/test_endpoints.sh http://localhost:8787

# Test against production
./scripts/test_endpoints.sh https://your-worker.workers.dev
```

### 3. CLI Integration Tests

Test all CLI commands:
```bash
./scripts/test_cli.sh

# Test specific CLI binary
./scripts/test_cli.sh ./target/debug/nasa-cli
```

### 4. Manual Testing

#### Worker Manual Testing

Start local development server:
```bash
wrangler dev
```

Test individual endpoints:
```bash
# APOD
curl "http://localhost:8787/api/apod"

# NEO Feed
curl "http://localhost:8787/api/neo/feed"

# Mars Photos
curl "http://localhost:8787/api/mars-photos/curiosity/photos?sol=1000"
```

#### CLI Manual Testing

```bash
# Build CLI in debug mode for testing
cargo build --features cli

# Test various commands
./target/debug/nasa-cli apod today
./target/debug/nasa-cli asteroids feed --start-date 2024-01-01
./target/debug/nasa-cli mars photos curiosity --sol 1000
```

## Testing Best Practices

### 1. Test Data Management

- The Cloudflare Worker handles API keys automatically
- Cache test responses to minimize API calls
- Use date ranges that are known to have data

### 2. Environment Setup

Set up test environment:
```bash
# Enable debug logging
export RUST_LOG=debug

# Use test configuration
export NASA_CLI_CONFIG_PATH=./tests/test_config.toml

# Disable cache for fresh results
export NASA_CLI_NO_CACHE=true
```

### 3. Performance Testing

Test response times:
```bash
# Time a request
time curl "http://localhost:8787/api/apod"

# Load test with multiple requests
for i in {1..10}; do
  curl -s "http://localhost:8787/api/apod" > /dev/null &
done
wait
```

### 4. Error Testing

Test error handling:
```bash
# Invalid date format
nasa apod date invalid-date

# Future date
nasa apod date 2099-01-01

# Invalid asteroid ID
nasa asteroids lookup invalid-id

# Missing required parameters
nasa earth image --lat 29.78
```

## Test Coverage

Check test coverage:
```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --all-features --out Html

# Open coverage report
open tarpaulin-report.html
```

## Continuous Integration

Tests run automatically on:
- Every push to main/develop branches
- Every pull request
- Tagged releases

GitHub Actions workflow includes:
- Formatting check (rustfmt)
- Linting (clippy)
- Unit tests
- Build verification

## Debugging Tests

### Enable Detailed Output

```bash
# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run specific test with debug output
cargo test test_apod_response -- --exact --nocapture
```

### Common Issues

1. **Rate Limit Errors**
   - Solution: Wait between tests
   - Enable caching to reduce API calls

2. **Network Timeouts**
   - Solution: Increase timeout in test scripts
   - Check internet connectivity

3. **Cache Permission Errors**
   - Solution: Ensure cache directory is writable
   - Clear cache: `nasa cache clear`

4. **Date-Related Failures**
   - Some endpoints don't have data for all dates
   - Use known good dates from NASA documentation

## Adding New Tests

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        // Arrange
        let input = "test data";
        
        // Act
        let result = your_function(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }

    #[tokio::test]
    async fn test_async_feature() {
        // Async test implementation
    }
}
```

### CLI Test Template

Add to `scripts/test_cli.sh`:
```bash
# Test new command
test_command "New feature" "new-command --option value -o json"
```

### Worker Endpoint Test Template

Add to `scripts/test_endpoints.sh`:
```bash
# Test new endpoint
test_endpoint "New Endpoint" "/api/new-endpoint" "param1=value1&param2=value2"
```

## Test Data

Common test data values:

- **Dates**: 2023-01-01 (known to have APOD data)
- **Asteroid ID**: 3542519 (known valid asteroid)
- **Mars Sol**: 1000 (has photos for all rovers)
- **Coordinates**: Houston (29.78, -95.33)
- **Exoplanet Query**: "select * from ps limit 5"

## Benchmarking

Run benchmarks:
```bash
# Add benchmark tests to your code
cargo bench

# Profile specific operation
cargo build --release --features cli
time ./target/release/nasa-cli apod today
```

## Security Testing

Test API key handling:
```bash
# Ensure API key is not logged
RUST_LOG=trace nasa apod today 2>&1 | grep -i "api_key"

# Test with invalid API key
NASA_API_KEY=invalid nasa apod today
```

## Contributing Tests

When submitting PRs:
1. Add tests for new features
2. Ensure all tests pass locally
3. Update test documentation
4. Add test commands to scripts if needed

## Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cloudflare Workers Testing](https://developers.cloudflare.com/workers/testing/)
- [NASA API Documentation](https://api.nasa.gov/)