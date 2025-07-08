# NASA-RS 🚀

[![CI/CD](https://github.com/marcusziade/nasa-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/marcusziade/nasa-rs/actions/workflows/ci.yml)
[![GitHub Pages](https://github.com/marcusziade/nasa-rs/actions/workflows/deploy-pages.yml/badge.svg)](https://github.com/marcusziade/nasa-rs/actions/workflows/deploy-pages.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Cloudflare Workers](https://img.shields.io/badge/Cloudflare-Workers-orange)](https://workers.cloudflare.com/)

A production-ready Rust Cloudflare Worker and CLI for accessing NASA's public APIs. This project provides a unified interface to all NASA APIs with built-in caching, rate limiting, and a beautiful command-line interface.

## Features

### Cloudflare Worker
- **Complete NASA API Coverage**: Implements ALL NASA public APIs
- **Smart Caching**: Cloudflare KV-based caching with endpoint-specific TTLs
- **Rate Limiting**: Per-IP rate limiting to prevent abuse
- **CORS Support**: Fully configured CORS headers
- **Error Handling**: Comprehensive error handling with meaningful messages
- **Performance**: Optimized for Cloudflare Workers runtime

### CLI Client
- **Beautiful Output**: Multiple output formats (JSON, Table, Pretty, CSV)
- **Response Caching**: Local file-based cache to minimize API calls
- **Configuration Management**: Store API endpoint and preferences
- **Interactive Mode**: Menu-driven interface for easy exploration
- **Interactive Features**: Progress bars, colored output, and prompts
- **Comprehensive Commands**: Full coverage of all NASA APIs

## Quick Start

### Worker Deployment

1. **Install Dependencies**
```bash
npm install -g wrangler
```

2. **Configure Secrets**
```bash
# Add your NASA API key
wrangler secret put NASA_API_KEY
# Enter: YOUR_NASA_API_KEY_HERE
```

3. **Deploy to Cloudflare**
```bash
wrangler deploy
```

### CLI Installation

```bash
# Build and install the CLI
cargo install --path . --features cli

# Or for development
cargo build --release --features cli
./target/release/nasa-cli --help
```

### Basic Usage

```bash
# Launch interactive mode for easy exploration
nasa interactive
# or use the alias
nasa i

# Get today's Astronomy Picture of the Day
nasa apod today

# Search for asteroids
nasa asteroids feed --start-date 2024-01-01 --end-date 2024-01-07

# Get Mars rover photos
nasa mars photos curiosity --sol 1000 --camera NAVCAM

# Search NASA's image library
nasa media search --query "apollo 11" --media-type image
```

## View Documentation

To view the interactive documentation and examples locally:

```bash
# From the project root, just run:
./scripts/serve

# Opens http://localhost:8000 in your browser
# Press Ctrl+C to stop
```

## Supported NASA APIs

<details>
<summary><strong>🌌 APOD - Astronomy Picture of the Day</strong></summary>

Get stunning space images with detailed explanations from professional astronomers.

```bash
# Today's picture
nasa apod today

# Specific date
nasa apod date 2023-07-20

# Random pictures
nasa apod random --count 5
```
</details>

<details>
<summary><strong>☄️ NeoWs - Near Earth Objects</strong></summary>

Track asteroids and their close approaches to Earth.

```bash
# Asteroid feed for date range
nasa asteroids feed --start-date 2024-01-01 --end-date 2024-01-07

# Lookup specific asteroid
nasa asteroids lookup 3542519

# Browse all asteroids
nasa asteroids browse --page 1 --size 20
```
</details>

<details>
<summary><strong>🌞 DONKI - Space Weather</strong></summary>

Monitor space weather events including solar flares, coronal mass ejections, and geomagnetic storms.

```bash
# Coronal Mass Ejections
nasa donki cme --start-date 2024-01-01

# Solar Flares
nasa donki flare --start-date 2024-01-01

# Geomagnetic Storms
nasa donki storm

# Space weather notifications
nasa donki notifications --type CME
```
</details>

<details>
<summary><strong>🔴 Mars Rover Photos</strong></summary>

Access photos from NASA's Mars rovers: Curiosity, Opportunity, and Spirit.

```bash
# Get photos by sol (Martian day)
nasa mars photos curiosity --sol 1000 --camera NAVCAM

# Get photos by Earth date
nasa mars photos curiosity --earth-date 2023-01-01

# Get latest photos
nasa mars latest curiosity

# Get mission manifest
nasa mars manifest curiosity
```

**Available Cameras:**
- **FHAZ**: Front Hazard Avoidance Camera
- **RHAZ**: Rear Hazard Avoidance Camera
- **MAST**: Mast Camera (Curiosity only)
- **CHEMCAM**: Chemistry and Camera Complex (Curiosity only)
- **MAHLI**: Mars Hand Lens Imager (Curiosity only)
- **MARDI**: Mars Descent Imager (Curiosity only)
- **NAVCAM**: Navigation Camera
- **PANCAM**: Panoramic Camera (Opportunity & Spirit only)
- **MINITES**: Miniature Thermal Emission Spectrometer (Opportunity & Spirit only)
</details>

<details>
<summary><strong>🌍 Earth Imagery</strong></summary>

Get Landsat 8 satellite imagery of Earth.

```bash
# Get satellite image (saves as PNG)
nasa earth image --lat 29.78 --lon -95.33 --date 2023-01-01

# Get available asset dates
nasa earth assets --lat 29.78 --lon -95.33 --date 2023-01-01
```
</details>

<details>
<summary><strong>🌏 EPIC - Earth Polychromatic Imaging Camera</strong></summary>

View Earth from the DSCOVR spacecraft's perspective.

```bash
# Get all available natural color dates
nasa epic natural all

# Get images for specific date
nasa epic natural 2023-12-25

# Enhanced color images
nasa epic enhanced 2023-12-25
```
</details>

<details>
<summary><strong>💡 Tech Transfer</strong></summary>

Search NASA's technology portfolio including patents, software, and spinoffs.

```bash
# Search patents
nasa tech patents --query "propulsion"

# Search software
nasa tech software --query "simulation"

# Search spinoff technologies
nasa tech spinoffs --query "medical"
```
</details>

<details>
<summary><strong>🖼️ NASA Image and Video Library</strong></summary>

Search NASA's extensive media collection.

```bash
# Basic search
nasa media search --query "apollo 11"

# Advanced search
nasa media search --query "mars" --media-type image --center JPL --year-start 2020

# Get asset details
nasa media asset NHQ202301230001
```

**NASA Centers:**
- **KSC**: Kennedy Space Center
- **JSC**: Johnson Space Center
- **JPL**: Jet Propulsion Laboratory
- **GSFC**: Goddard Space Flight Center
- **ARC**: Ames Research Center
- **HQ**: NASA Headquarters
</details>

<details>
<summary><strong>🪐 Exoplanet Archive</strong></summary>

Query the NASA Exoplanet Archive using ADQL.

```bash
# Custom ADQL query
nasa exoplanets search "select * from ps where pl_masse < 10"

# Search Kepler discoveries
nasa exoplanets kepler

# Search by planet name pattern
nasa exoplanets kepler --name "452"
```
</details>

<details>
<summary><strong>☄️ SSD/CNEOS - Solar System Dynamics</strong></summary>

Access data about asteroids, comets, and other small bodies.

```bash
# Close approach data
nasa ssd close-approach --date-min 2024-01-01 --date-max 2024-12-31 --dist-max 0.05

# Potentially hazardous asteroids only
nasa ssd close-approach --pha

# Fireball atmospheric impacts
nasa ssd fireballs --date-min 2023-01-01 --req-loc

# Sentry impact risk assessment
nasa ssd sentry --object "99942 Apophis"
```
</details>

## Configuration

### CLI Configuration

Initialize configuration:
```bash
nasa config init
```

Configuration file location: `~/.config/nasa-cli/config.toml`

```toml
api_endpoint = "https://nasa-api.workers.dev"
output_format = "pretty"  # json, table, pretty, csv
use_cache = true
cache_dir = "~/.config/nasa-cli/cache"
cache_ttl_minutes = 60
```

Set configuration values:
```bash
nasa config set api_endpoint https://your-worker.workers.dev
nasa config set output_format table
nasa config set use_cache false
```

Show current configuration:
```bash
nasa config show
```

### Cache Management

Clear cache:
```bash
nasa cache clear
```

Show cache statistics:
```bash
nasa cache stats
```

### Output Formats

Control output format with the `-o` flag:
```bash
# JSON output
nasa apod today -o json

# Table format (great for lists)
nasa asteroids browse -o table

# CSV format (for data analysis)
nasa ssd fireballs -o csv > fireballs.csv

# Pretty format (default, human-readable)
nasa apod today -o pretty
```

## Interactive Mode

The CLI includes a powerful interactive mode that makes exploring NASA APIs easy and intuitive:

```bash
# Launch interactive mode
nasa interactive
```

Features:
- **Menu Navigation**: Browse all NASA APIs through an intuitive menu system
- **Guided Input**: Interactive prompts for all parameters with validation
- **Smart Defaults**: Sensible defaults for common queries
- **Date Picker**: Easy date selection with format validation
- **Multi-Select**: Choose multiple options where applicable (e.g., Mars rover cameras)
- **Live Settings**: Change output format and cache settings on the fly
- **Paginated Output**: Long results are automatically paginated

### Interactive Mode Examples

1. **Exploring Mars Photos**:
   - Select "Mars Rover Photos" from the main menu
   - Choose a rover (Curiosity, Opportunity, or Spirit)
   - Select photo search type (by sol, Earth date, or latest)
   - Pick cameras using multi-select
   - View results in your preferred format

2. **Space Weather Monitoring**:
   - Select "Space Weather - DONKI"
   - Choose event type (CME, Solar Flares, etc.)
   - Optionally specify date range
   - View real-time space weather data

3. **Asteroid Tracking**:
   - Select "Asteroids - Near Earth Objects"
   - Choose feed type or lookup specific asteroid
   - Filter by date range or hazardous status
   - Export results to CSV for analysis

## Advanced Usage

### No-Cache Mode

Bypass cache for fresh data:
```bash
nasa apod today --no-cache
```

### Custom API Endpoint

Use a different API endpoint:
```bash
nasa apod today --endpoint https://custom-api.example.com
```

### Examples by Use Case

<details>
<summary><strong>Educational Astronomy Content</strong></summary>

```bash
# Get a week of APOD images for a presentation
nasa apod date 2024-01-01 > monday.json
nasa apod date 2024-01-02 > tuesday.json
# ... continue for the week

# Get random space images for a quiz
nasa apod random --count 10 -o json > quiz_images.json
```
</details>

<details>
<summary><strong>Asteroid Tracking</strong></summary>

```bash
# Check this week's close approaches
nasa asteroids feed -o table

# Monitor a specific asteroid
nasa asteroids lookup 99942  # Apophis

# Export potentially hazardous asteroids data
nasa ssd close-approach --pha -o csv > hazardous_asteroids.csv
```
</details>

<details>
<summary><strong>Mars Exploration</strong></summary>

```bash
# Get recent Curiosity photos
nasa mars latest curiosity -o json | jq '.photos[].img_src'

# Compare terrain over time
nasa mars photos curiosity --earth-date 2023-01-01 --camera NAVCAM
nasa mars photos curiosity --earth-date 2024-01-01 --camera NAVCAM
```
</details>

<details>
<summary><strong>Space Weather Monitoring</strong></summary>

```bash
# Check recent solar activity
nasa donki flare --start-date $(date -d '7 days ago' +%Y-%m-%d)

# Monitor for geomagnetic storms
nasa donki storm --start-date $(date -d '30 days ago' +%Y-%m-%d)

# Get all space weather notifications
nasa donki notifications -o json
```
</details>

## Troubleshooting

### Common Issues

**Rate Limit Exceeded**
```
Error: API error 429: Rate limit exceeded
```
Solution: Wait a few minutes before retrying. The API rate limit is enforced by the Cloudflare Worker.

**Cache Permission Errors**
```
Error: Permission denied (os error 13)
```
Solution: Ensure the cache directory is writable:
```bash
mkdir -p ~/.config/nasa-cli/cache
chmod 755 ~/.config/nasa-cli/cache
```

**Invalid Date Format**
```
Error: Invalid date format
```
Solution: Use YYYY-MM-DD format for all dates.

### Debug Mode

Set the `RUST_LOG` environment variable for debug output:
```bash
RUST_LOG=debug nasa apod today
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/nasa-rs
cd nasa-rs

# Build the worker
cargo build --release

# Build the CLI
cargo build --release --features cli

# Run tests
cargo test --all-features
```

### Local Worker Development

```bash
# Start local development server
wrangler dev

# Test with local CLI
nasa config set api_endpoint http://localhost:8787
nasa apod today
```

## Performance Tips

1. **Use caching**: Keep caching enabled for frequently accessed data
2. **Batch requests**: Use date ranges instead of multiple individual requests
3. **Choose appropriate output formats**: Use JSON for programmatic access, tables for lists
4. **Leverage local cache**: The CLI caches responses locally to minimize API calls

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- NASA for providing these amazing public APIs
- Cloudflare for the Workers platform
- The Rust community for excellent libraries and tools

---

Built with ❤️ by the Rust community