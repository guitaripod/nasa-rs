# NASA CLI Manual

A comprehensive guide to using the NASA CLI with all available commands, options, and example outputs.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Commands Overview](#commands-overview)
- [APOD (Astronomy Picture of the Day)](#apod-astronomy-picture-of-the-day)
- [Asteroids (NeoWs)](#asteroids-neows)
- [DONKI (Space Weather)](#donki-space-weather)
- [Earth Imagery](#earth-imagery)
- [EPIC (Earth Polychromatic Imaging Camera)](#epic-earth-polychromatic-imaging-camera)
- [Mars Rover Photos](#mars-rover-photos)
- [Exoplanets](#exoplanets)
- [Tech Transfer](#tech-transfer)
- [Media Library](#media-library)
- [SSD/CNEOS (Solar System Dynamics)](#ssdcneos-solar-system-dynamics)

## Installation

```bash
# Build the CLI
cargo build --release

# The binary will be available at
./target/release/nasa-cli
```

## Configuration

### Using Custom Endpoint

You can specify a custom API endpoint (like the Cloudflare Worker) using the `--endpoint` flag:

```bash
nasa-cli --endpoint https://nasa-api-worker.guitaripod.workers.dev <command>
```

### Global Options

- `-o, --output <format>`: Output format (json, table, pretty, csv)
- `--endpoint <url>`: API endpoint URL
- `--no-cache`: Disable response caching
- `-h, --help`: Print help information

## Commands Overview

```bash
$ nasa-cli --help
```

<details>
<summary>Click to expand output</summary>

```
NASA API CLI

Usage: nasa-cli [OPTIONS] [COMMAND]

Commands:
  apod        Astronomy Picture of the Day
  asteroids   Near Earth Objects
  donki       Space Weather Database
  earth       Landsat 8 imagery
  epic        Earth Polychromatic Imaging Camera
  mars        Mars rover photos
  exoplanets  Exoplanet data
  tech        NASA technology transfer
  media       NASA Image and Video Library
  ssd         Solar System Dynamics
  help        Print this message or the help of the given subcommand(s)

Options:
  -o, --output <output>      Output format [possible values: json, table, pretty, csv]
      --endpoint <endpoint>  API endpoint URL
      --no-cache             Disable response caching
  -h, --help                 Print help
  -V, --version              Print version
```

</details>

## APOD (Astronomy Picture of the Day)

### Get Today's Picture

```bash
$ nasa-cli apod today
```

<details>
<summary>Click to expand output</summary>

```
Perihelion Pangea
────────────────────────────────────────
Date: 2025-07-08
URL: https://apod.nasa.gov/apod/image/2107/APOD_Perihelion_Insta1024c.jpg
Explanation: Warm shades and subtle colors of the Sun are captured in this
high-resolution close-up of the Pihelion's active regions near the edge of the Sun.
```

</details>

### Get Picture by Date

```bash
$ nasa-cli apod date 2025-07-04
```

<details>
<summary>Click to expand output</summary>

```
Stars, Dust, Pillars, and Jets in the Pelican Nebula
────────────────────────────────────────
Date: 2025-07-04
URL: https://apod.nasa.gov/apod/image/2107/PelicanNebula_Symon_960.jpg
Explanation: What dark structures arise within the Pelican Nebula? On the whole, the
nebula appears like a bird (a pelican) and is seen toward the constellation of a
different bird: Cygnus, a Swan.
```

</details>

### Get Random Picture

```bash
$ nasa-cli apod random
```

<details>
<summary>Click to expand output</summary>

```
The Moons of Mars
────────────────────────────────────────
Date: 2025-05-17
URL: https://apod.nasa.gov/apod/image/1705/PIA21474_fig1MarsGlobalMoonsEnhanced1024.jpg
Media Type: image
Explanation: Mars has two tiny moons, Phobos and Deimos, named for the figures in Greek
mythology Fear and Panic.
```

</details>

### Get APODs for Date Range

```bash
$ nasa-cli apod range 2025-01-01 2025-01-03
```

<details>
<summary>Click to expand output</summary>

```
Found 3 items
────────────────────────────────────────

► Item 1
Title: Alpha Centauri: The Closest Star System
Date: 2025-01-01
URL: https://apod.nasa.gov/apod/image/2501/AlphaCen_Cantrell_960.jpg
Media Type: image
Copyright: Telescope Live, Heaven's Mirror Observatory; Processing: Chris Cantrell

► Item 2
Title: Solar Analemma 2024
Date: 2025-01-02
URL: https://apod.nasa.gov/apod/image/2501/solaranalemma2024-4-5BetulT1024.jpeg
Media Type: image
Copyright: Betul Turksoy

► Item 3
Title: Eclipse Pair
Date: 2025-01-03
URL: https://apod.nasa.gov/apod/image/2501/APODEclipsePair1024.jpg
Media Type: image
Copyright: Josh Dury
```

</details>

### Batch APOD Retrieval

```bash
$ nasa-cli apod batch --count 5 --thumbs
```

<details>
<summary>Click to expand output</summary>

```
count: 5
images: [5 random APOD entries with thumbnail URLs included]
```

</details>

### JSON Output

```bash
$ nasa-cli apod today -o json
```

<details>
<summary>Click to expand output</summary>

```json
{
  "copyright": "Dennis Simmons",
  "date": "2025-07-08",
  "explanation": "Perihelion for 2021 is today, January 2, at 13:51 UTC. That's the closest point...",
  "hdurl": "https://apod.nasa.gov/apod/image/2107/APOD_Perihelion_Insta.jpg",
  "media_type": "image",
  "service_version": "v1",
  "title": "Perihelion Pangea",
  "url": "https://apod.nasa.gov/apod/image/2107/APOD_Perihelion_Insta1024c.jpg"
}
```

</details>

### Table Output

```bash
$ nasa-cli apod today -o table
```

<details>
<summary>Click to expand output</summary>

```
+-----------------+------------------------------------------------------------------+
| Field           | Value                                                            |
+-----------------+------------------------------------------------------------------+
| copyright       | Dennis Simmons                                                   |
| date            | 2025-07-08                                                       |
| explanation     | Perihelion for 2021 is today, January 2, at 13:51 UTC. That's...| 
| hdurl           | https://apod.nasa.gov/apod/image/2107/APOD_Perihelion_Insta.jpg |
| media_type      | image                                                            |
| service_version | v1                                                               |
| title           | Perihelion Pangea                                                |
| url             | https://apod.nasa.gov/apod/image/2107/APOD_Perihelion_Insta1... |
+-----------------+------------------------------------------------------------------+
```

</details>

## Asteroids (NeoWs)

### Get Asteroid Feed

```bash
$ nasa-cli asteroids feed
```

<details>
<summary>Click to expand output</summary>

```
element_count: 121
links: {3 fields}
near_earth_objects: {8 fields}
```

</details>

### Get Asteroid Feed with Date Range

```bash
$ nasa-cli asteroids feed 2025-07-07 2025-07-08
```

<details>
<summary>Click to expand output</summary>

```
element_count: 42
links: {3 fields}
near_earth_objects: {2 fields}
```

</details>

### Lookup Specific Asteroid

```bash
$ nasa-cli asteroids lookup 3542519
```

<details>
<summary>Click to expand output</summary>

```
absolute_magnitude_h: 24.51
close_approach_data: [44 items]
designation: 2010 PK9
estimated_diameter: {4 fields}
id: 3542519
is_potentially_hazardous_asteroid: false
is_sentry_object: false
links: {1 fields}
name: (2010 PK9)
name_limited: 2010 PK9
nasa_jpl_url: https://ssd.jpl.nasa.gov/tools/sbdb_lookup.html#/?sstr=3542519
neo_reference_id: 3542519
orbital_data: {17 fields}
```

</details>

### Browse Asteroids

```bash
$ nasa-cli asteroids browse
```

<details>
<summary>Click to expand output</summary>

```
links: {3 fields}
near_earth_objects: [20 items]
page: {4 fields}
```

</details>

### Table Output for Asteroid Feed

```bash
$ nasa-cli asteroids feed -o table
```

<details>
<summary>Click to expand output</summary>

```
+--------------------+-------------+
| Field              | Value       |
+--------------------+-------------+
| element_count      | 121         |
| links              | {3 fields}  |
| near_earth_objects | {8 fields}  |
+--------------------+-------------+
```

</details>

### Extended Feed with Filters

```bash
$ nasa-cli asteroids feed-extended --hazardous --size medium --max-distance 0.1
```

<details>
<summary>Click to expand output</summary>

```
element_count: 3
filters_applied: {
  "hazardous": true,
  "size": "medium",
  "distance_range": {
    "min": null,
    "max": "0.1"
  }
}
asteroids: [
  {
    "name": "(2021 AF8)",
    "is_potentially_hazardous_asteroid": true,
    "estimated_diameter": {
      "kilometers": {
        "estimated_diameter_max": 0.5303
      }
    },
    "close_approach_data": [...]
  },
  // ... more filtered asteroids
]
```

</details>

### Batch Asteroid Lookup

```bash
$ nasa-cli asteroids batch-lookup 3542519 2153306 54379375
```

<details>
<summary>Click to expand output</summary>

```
count: 3
asteroids: [
  {
    "id": "3542519",
    "data": {
      "name": "(2010 PK9)",
      "absolute_magnitude_h": 24.51,
      // ... full asteroid data
    }
  },
  {
    "id": "2153306",
    "data": {
      "name": "(153306) 2001 JL1",
      // ... full asteroid data
    }
  },
  {
    "id": "54379375",
    "data": {
      "name": "(2023 VD3)",
      // ... full asteroid data
    }
  }
]
```

</details>

### Weekly Close Approach Summary

```bash
$ nasa-cli asteroids weekly-summary
```

<details>
<summary>Click to expand output</summary>

```
week_of: 2025-07-07
week_ending: 2025-07-13
summary: {
  "total_asteroids": 83,
  "hazardous_asteroids": 5,
  "closest_approach": {
    "distance_au": 0.0101566751,
    "asteroid": "(2025 MC92)"
  }
}
element_count: 83
```

</details>

### Weekly Summary for Next Week

```bash
$ nasa-cli asteroids weekly-summary 1
```

<details>
<summary>Click to expand output</summary>

```
week_of: 2025-07-14
week_ending: 2025-07-20
summary: {
  "total_asteroids": 67,
  "hazardous_asteroids": 4,
  "closest_approach": {
    "distance_au": 0.0089234,
    "asteroid": "(2025 BA)"
  }
}
```

</details>

## DONKI (Space Weather)

### CME (Coronal Mass Ejection) Events

```bash
$ nasa-cli donki cme list
```

<details>
<summary>Click to expand output</summary>

```
Found 3 items
────────────────────────────────────────

► Item 1
Activity ID: 2025-07-05T18:00:00-CME-001
Catalog: M2M_CATALOG
Start Time: 2025-07-05T18:00:00Z
Source Location: S16E999
Active Region Number: null
Link: https://webtools.ccmc.gsfc.nasa.gov/DONKI/view/CME/31609/-1
Note: Eruption on the SW limb/backside starting around 17:00Z
Analysis: [3 items]
Linked Events: null


► Item 2
Activity ID: 2025-07-05T04:25:00-CME-001
Catalog: M2M_CATALOG
Start Time: 2025-07-05T04:25:00Z
Source Location: N17W50
Active Region Number: 13869
Link: https://webtools.ccmc.gsfc.nasa.gov/DONKI/view/CME/31601/-1
Note: Associated with the M2.3 class flare from AR 3869 peaking at 04:06Z
Analysis: [2 items]
Linked Events: [1 items]
```

</details>

### Solar Flares

```bash
$ nasa-cli donki solar-flares
```

<details>
<summary>Click to expand output</summary>

```
Found 15 items
────────────────────────────────────────

► Item 1
Flare ID: 2025-07-06T18:39:00-FLR-001
Instruments: [2 items]
Begin Time: 2025-07-06T18:39:00Z
Peak Time: 2025-07-06T18:48:00Z
End Time: 2025-07-06T18:56:00Z
Class Type: C4.5
Source Location: N19E52
Active Region Number: 3869
Linked Events: null
Link: https://webtools.ccmc.gsfc.nasa.gov/DONKI/view/FLR/31640/-1


► Item 2
Flare ID: 2025-07-06T06:01:00-FLR-001
Instruments: [2 items]
Begin Time: 2025-07-06T06:01:00Z
Peak Time: 2025-07-06T06:10:00Z
End Time: 2025-07-06T06:17:00Z
Class Type: C8.8
Source Location: N19E51
Active Region Number: 3869
Linked Events: null
Link: https://webtools.ccmc.gsfc.nasa.gov/DONKI/view/FLR/31632/-1
```

</details>

### Solar Flares with Table Output

```bash
$ nasa-cli donki solar-flares -o table
```

<details>
<summary>Click to expand output</summary>

```
+---------------------+-----------+------------+------------+---------------------+-----------------+
| Begin Time          | Peak Time | Class Type | Location   | Active Region       | Instruments     |
+---------------------+-----------+------------+------------+---------------------+-----------------+
| 2025-07-06T18:39:00Z| 18:48:00Z | C4.5       | N19E52     | 3869                | GOES-P: GOES-18 |
| 2025-07-06T06:01:00Z| 06:10:00Z | C8.8       | N19E51     | 3869                | GOES-P: GOES-18 |
| 2025-07-05T23:55:00Z| 00:32:00Z | C5.0       | N18E50     | 3869                | GOES-P: GOES-18 |
| 2025-07-05T21:58:00Z| 22:12:00Z | C5.3       | N16E48     | 3869                | GOES-P: GOES-18 |
| 2025-07-05T18:55:00Z| 19:12:00Z | C4.6       | N16E48     | 3869                | GOES-P: GOES-18 |
+---------------------+-----------+------------+------------+---------------------+-----------------+
```

</details>

### Geomagnetic Storms

```bash
$ nasa-cli donki gst
```

<details>
<summary>Click to expand output</summary>

```
No geomagnetic storms found
```

</details>

### Solar Energetic Particles

```bash
$ nasa-cli donki sep
```

<details>
<summary>Click to expand output</summary>

```
No solar energetic particle events found
```

</details>

## Earth Imagery

### Get Earth Imagery

```bash
$ nasa-cli earth imagery 40.7128 -74.0060
```

<details>
<summary>Click to expand output</summary>

```
Error: API error 500 Internal Server Error: NASA Earth Imagery API error
```

*Note: The Earth API is currently experiencing issues on NASA's servers*

### Assets Lookup

```bash
$ nasa-cli earth assets 40.7128 -74.0060
```

<details>
<summary>Click to expand output</summary>

```
Error: API error 500 Internal Server Error: NASA Earth Imagery API error
```

## EPIC (Earth Polychromatic Imaging Camera)

### List All Available Dates

```bash
$ nasa-cli epic natural all
```

<details>
<summary>Click to expand output</summary>

```
Found 3291 items
────────────────────────────────────────

► Item 1
Date: 2025-07-06


► Item 2
Date: 2025-07-05


► Item 3
Date: 2025-07-04


► Item 4
Date: 2025-07-03


► Item 5
Date: 2025-07-02

[... truncated for brevity ...]
```

</details>

### Natural Color Images for Specific Date

```bash
$ nasa-cli epic natural 2025-07-06
```

<details>
<summary>Click to expand output</summary>

```
Found 21 items
────────────────────────────────────────

► Item 1
Date: 2025-07-06 00:59:48
attitude_quaternions: {4 fields}
caption: This image was taken by NASA's EPIC camera onboard the NOAA DSCOVR spacecraft
centroid_coordinates: {2 fields}
coords: {5 fields}
dscovr_j2000_position: {3 fields}
identifier: 20250706010437
image: epic_1b_20250706010437
lunar_j2000_position: {3 fields}
sun_j2000_position: {3 fields}
version: 03


► Item 2
Date: 2025-07-06 02:05:16
attitude_quaternions: {4 fields}
caption: This image was taken by NASA's EPIC camera onboard the NOAA DSCOVR spacecraft
centroid_coordinates: {2 fields}
coords: {5 fields}
dscovr_j2000_position: {3 fields}
identifier: 20250706021004
image: epic_1b_20250706021004
lunar_j2000_position: {3 fields}
sun_j2000_position: {3 fields}
version: 03

[... more items ...]
```

</details>

### Enhanced Color Images

```bash
$ nasa-cli epic enhanced 2025-07-05
```

<details>
<summary>Click to expand output</summary>

```
Found 22 items
────────────────────────────────────────

► Item 1
Date: 2025-07-05 00:41:06
attitude_quaternions: {4 fields}
caption: This image was taken by NASA's EPIC camera onboard the NOAA DSCOVR spacecraft
centroid_coordinates: {2 fields}
coords: {5 fields}
dscovr_j2000_position: {3 fields}
identifier: 20250705004554
image: epic_RGB_20250705004554
lunar_j2000_position: {3 fields}
sun_j2000_position: {3 fields}
version: 03

[... more items ...]
```

</details>

### Bulk Image Retrieval for Date Range

```bash
$ nasa-cli epic bulk natural --start-date 2025-01-01 --end-date 2025-01-02 --limit 2
```

<details>
<summary>Click to expand output</summary>

```
count: 4
start_date: 2025-01-01
end_date: 2025-01-02
type: natural
images: [
  {
    "date": "2025-01-01 00:13:03",
    "identifier": "20250101001751",
    "image": "epic_1b_20250101001751",
    "caption": "This image was taken by NASA's EPIC camera onboard the NOAA DSCOVR spacecraft",
    // ... coordinate data
  },
  // ... more images
]
```

</details>

### Monthly Archive

```bash
$ nasa-cli epic archive enhanced 2025 1
```

<details>
<summary>Click to expand output</summary>

```
count: 672
year: 2025
month: 1
type: enhanced
images: [all enhanced images from January 2025]
```

</details>

### Download Image Set

```bash
$ nasa-cli epic download-set natural 2025-01-01 --save-to ./epic-images --format png
```

<details>
<summary>Click to expand output</summary>

```
status: download_planned
count: 22
date: 2025-01-01
type: natural
directory: ./epic-images
```

*Note: This currently shows what would be downloaded. Actual download functionality can be implemented.*

</details>

## Mars Rover Photos

### Mission Manifest

```bash
$ nasa-cli mars manifest curiosity
```

<details>
<summary>Click to expand output</summary>

```
photo_manifest: {8 fields}
```

</details>

### Mission Manifest with Pretty Output

```bash
$ nasa-cli mars manifest curiosity -o json | jq .photo_manifest
```

<details>
<summary>Click to expand output</summary>

```json
{
  "landing_date": "2012-08-06",
  "launch_date": "2011-11-26",
  "max_date": "2025-07-07",
  "max_sol": 4592,
  "name": "Curiosity",
  "photos": [
    {
      "cameras": ["CHEMCAM", "FHAZ", "MARDI", "RHAZ"],
      "earth_date": "2012-08-06",
      "sol": 0,
      "total_photos": 3702
    }
    // ... more sols
  ],
  "status": "active",
  "total_photos": 704569
}
```

</details>

### Latest Photos

```bash
$ nasa-cli mars latest curiosity
```

<details>
<summary>Click to expand output</summary>

```
latest_photos: [2 items]
```

</details>

### Latest Photos with Details

```bash
$ nasa-cli mars latest curiosity -o json | jq '.latest_photos[] | {id, sol, camera: .camera.name, earth_date}'
```

<details>
<summary>Click to expand output</summary>

```json
{
  "id": 1328410,
  "sol": 4592,
  "camera": "CHEMCAM_RMI",
  "earth_date": "2025-07-07"
}
{
  "id": 1328411,
  "sol": 4592,
  "camera": "CHEMCAM_RMI",
  "earth_date": "2025-07-07"
}
```

</details>

### Photos by Sol

```bash
$ nasa-cli mars photos curiosity 4591
```

<details>
<summary>Click to expand output</summary>

```
photos: [25 items]
```

</details>

### Photos by Earth Date

```bash
$ nasa-cli mars photos curiosity -- 2025-07-06
```

<details>
<summary>Click to expand output</summary>

```
photos: [12 items]
```

</details>

### Photos by Earth Date and Camera

```bash
$ nasa-cli mars photos curiosity -- 2025-07-06 NAVCAM
```

<details>
<summary>Click to expand output</summary>

```
photos: [12 items]
```

</details>

### Opportunity Rover Photos

```bash
$ nasa-cli mars photos opportunity 5111
```

<details>
<summary>Click to expand output</summary>

```
photos: [1 items]
```

</details>

### Batch Photo Retrieval by Sol Range

```bash
$ nasa-cli mars batch curiosity --sol-start 3000 --sol-end 3001 --limit 5
```

<details>
<summary>Click to expand output</summary>

```
count: 5
photos: [
  {
    "id": 787577,
    "sol": 3000,
    "camera": {
      "name": "FHAZ",
      "full_name": "Front Hazard Avoidance Camera"
    },
    "earth_date": "2021-01-13",
    "img_src": "https://mars.nasa.gov/msl-raw-images/..."
  },
  // ... more photos
]
```

</details>

### Collection with Multiple Cameras

```bash
$ nasa-cli mars collection curiosity 3000 --cameras FHAZ,RHAZ,NAVCAM --all-pages
```

<details>
<summary>Click to expand output</summary>

```
count: 150
sol: 3000
rover: curiosity
photos: [150 items from all specified cameras]
```

</details>

### Download Mars Photos

```bash
$ nasa-cli mars download curiosity 3000 --save-to ./mars-sol-3000 --camera NAVCAM
```

<details>
<summary>Click to expand output</summary>

```
status: download_planned
count: 12
directory: ./mars-sol-3000
```

*Note: This currently shows what would be downloaded. Actual download functionality can be implemented.*

</details>

## Exoplanets

### Search Exoplanets

```bash
$ nasa-cli exoplanets search 'select pl_name from pscomppars limit 5'
```

<details>
<summary>Click to expand output</summary>

```
Error: API error 500 Internal Server Error: Internal Server Error
```

*Note: The Exoplanets API is currently experiencing issues*

### Kepler Discoveries

```bash
$ nasa-cli exoplanets kepler
```

<details>
<summary>Click to expand output</summary>

```
Error: Command timed out after 2m 0.0s
```

## Tech Transfer

### Search Patents

```bash
$ nasa-cli tech patents "propulsion"
```

<details>
<summary>Click to expand output</summary>

```
count: 1
page: 0
perpage: 10
results: [1 items]
total: 1
```

</details>

### Search Software

```bash
$ nasa-cli tech software "analysis"
```

<details>
<summary>Click to expand output</summary>

```
count: 9
page: 0
perpage: 10
results: [9 items]
total: 9
```

</details>

### Search Spinoffs

```bash
$ nasa-cli tech spinoffs "medical"
```

<details>
<summary>Click to expand output</summary>

```
count: 3
page: 0
perpage: 10
results: [3 items]
total: 3
```

</details>

### Software Results Structure

```bash
$ nasa-cli tech software "analysis" -o json | jq '.results[0]'
```

<details>
<summary>Click to expand output</summary>

```json
[
  "13579",
  "docket_number",
  "NASA-CASE-ARC-17578-1",
  "patent_number",
  "10,101,473 B2",
  "application_sn",
  "15/616,213",
  "title",
  "GPS/GNSS Interference Signal Analysis System",
  "patent_issue_date",
  "October 16, 2018",
  "innovator",
  [
    {
      // innovator details
    }
  ]
]
```

</details>

## Media Library

### Search Media

```bash
$ nasa-cli media search "apollo 11"
```

<details>
<summary>Click to expand output</summary>

```
collection: {4 fields}
```

</details>

### Search Media - Mars

```bash
$ nasa-cli media search "mars"
```

<details>
<summary>Click to expand output</summary>

```
collection: {4 fields}
```

</details>

### Media Search Structure

```bash
$ nasa-cli media search "mars" -o json | jq '.collection | keys'
```

<details>
<summary>Click to expand output</summary>

```json
[
  "href",
  "items",
  "metadata",
  "version"
]
```

</details>

### Get Asset Details

```bash
$ nasa-cli media asset NHQ_2019_0311_Go_Forward_to_the_Moon
```

<details>
<summary>Click to expand output</summary>

```
Error: API error 500 Internal Server Error: Internal Server Error
```

*Note: The Media Asset endpoint is currently experiencing issues*

</details>

### Collection Search (All Pages)

```bash
$ nasa-cli media collection apollo11 --all-pages --limit 100
```

<details>
<summary>Click to expand output</summary>

```
query: apollo11
total_items: 100
collection: {
  items: [100 items matching apollo11]
}
```

</details>

### Batch Asset Retrieval

```bash
$ nasa-cli media batch-assets as11-40-5903 as11-40-5877 as11-40-5875
```

<details>
<summary>Click to expand output</summary>

```
count: 3
assets: [
  {
    "nasa_id": "as11-40-5903",
    "asset": {
      // asset data
    }
  },
  {
    "nasa_id": "as11-40-5877",
    "asset": {
      // asset data
    }
  },
  {
    "nasa_id": "as11-40-5875",
    "asset": {
      // asset data
    }
  }
]
```

</details>

### Download Search Results

```bash
$ nasa-cli media download-results "mars rover" --media-type image --limit 50 --save-to ./nasa-images
```

<details>
<summary>Click to expand output</summary>

```
status: download_planned
query: mars rover
count: 50
directory: ./nasa-images
```

*Note: This currently shows what would be downloaded. Actual download functionality can be implemented.*

## SSD/CNEOS (Solar System Dynamics)

### Close Approach Data

```bash
$ nasa-cli ssd close-approach
```

<details>
<summary>Click to expand output</summary>

```
Error: API error 500 Internal Server Error: Internal Server Error
```

### Fireballs

```bash
$ nasa-cli ssd fireballs
```

<details>
<summary>Click to expand output</summary>

```
Error: API error 500 Internal Server Error: Internal Server Error
```

### Sentry Impact Risk

```bash
$ nasa-cli ssd sentry
```

<details>
<summary>Click to expand output</summary>

```
Error: API error 500 Internal Server Error: Internal Server Error
```

*Note: All SSD/CNEOS endpoints are currently experiencing issues on NASA's servers*

## Error Handling

The CLI provides clear error messages when issues occur:

- **API Errors**: Display the HTTP status code and error message
- **Network Errors**: Show connection issues
- **Invalid Arguments**: Display usage help
- **Rate Limiting**: NASA APIs have rate limits; the CLI will show appropriate messages

## Tips and Best Practices

1. **Use Output Formats**: Different formats work better for different use cases:
   - `json`: Best for piping to `jq` or other tools
   - `table`: Good for quick viewing in terminal
   - `pretty`: Human-readable format with nice formatting
   - `csv`: For importing into spreadsheets

2. **Date Formats**: Most commands accept dates in `YYYY-MM-DD` format

3. **Caching**: The CLI caches responses by default. Use `--no-cache` to force fresh data

4. **Custom Endpoints**: Use the `--endpoint` flag to point to your own NASA API proxy or the Cloudflare Worker

5. **Pagination**: Some endpoints support pagination (like Mars photos). Check command help for page parameters

6. **Rate Limits**: Be mindful of NASA's API rate limits. The official limit is 1000 requests per hour per IP

## Troubleshooting

### Command Not Found
Make sure you're running the binary from the correct location:
```bash
./target/release/nasa-cli
```

### API Errors
Some NASA APIs may experience temporary issues. If you get 500 errors, try:
1. Waiting a few minutes and retrying
2. Checking if the issue is with specific endpoints
3. Using the official NASA API directly to verify

### Build Issues
If you have trouble building:
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Network Issues
If you're behind a proxy or firewall, you may need to configure your network settings appropriately.