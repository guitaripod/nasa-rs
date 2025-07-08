use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm, MultiSelect};
use colored::Colorize;
use chrono::{Local, NaiveDate};
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::{
    api::ApiClient,
    output::{Formatter, OutputFormat},
    commands::CommandContext,
};

pub async fn run_interactive_mode(context: &CommandContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "🚀 NASA API Interactive Explorer".bright_cyan().bold());
    println!("{}", "================================".bright_cyan());
    
    let client = ApiClient::new(
        context.config.api_endpoint.clone(),
        context.config.api_key.clone(),
        context.config.cache_dir.clone(),
        context.config.use_cache,
        context.config.cache_ttl_minutes,
    );
    
    loop {
        let apis = vec![
            "🌌 APOD - Astronomy Picture of the Day",
            "☄️ Asteroids - Near Earth Objects",
            "🌞 Space Weather - DONKI",
            "🔴 Mars Rover Photos",
            "🌍 Earth Satellite Imagery",
            "🌏 EPIC - Earth from Space",
            "💡 Tech Transfer - NASA Technology",
            "🖼️ Media Library - Images & Videos",
            "🪐 Exoplanets - Alien Worlds",
            "☄️ SSD/CNEOS - Solar System Dynamics",
            "⚙️ Settings",
            "❌ Exit",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a NASA API to explore")
            .items(&apis)
            .default(0)
            .interact()?;
        
        match selection {
            0 => explore_apod(&client, &context.config.output_format).await?,
            1 => explore_asteroids(&client, &context.config.output_format).await?,
            2 => explore_space_weather(&client, &context.config.output_format).await?,
            3 => explore_mars(&client, &context.config.output_format).await?,
            4 => explore_earth(&client, &context.config.output_format).await?,
            5 => explore_epic(&client, &context.config.output_format).await?,
            6 => explore_tech(&client, &context.config.output_format).await?,
            7 => explore_media(&client, &context.config.output_format).await?,
            8 => explore_exoplanets(&client, &context.config.output_format).await?,
            9 => explore_ssd(&client, &context.config.output_format).await?,
            10 => configure_settings(context).await?,
            11 => {
                println!("\n{}", "👋 Thanks for exploring NASA APIs!".bright_green());
                break;
            }
            _ => unreachable!(),
        }
        
        println!();
        if !Confirm::new()
            .with_prompt("Continue exploring?")
            .default(true)
            .interact()? {
            println!("\n{}", "👋 Thanks for exploring NASA APIs!".bright_green());
            break;
        }
    }
    
    Ok(())
}

async fn explore_apod(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "🌌 Astronomy Picture of the Day".bright_yellow().bold());
    
    let options = vec![
        "Today's Picture",
        "Specific Date",
        "Random Pictures",
        "Date Range",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to see?")
        .items(&options)
        .interact()?;
    
    let data = match selection {
        0 => {
            // Today's picture
            client.get("/api/apod", HashMap::new()).await?
        }
        1 => {
            // Specific date
            let date: String = Input::new()
                .with_prompt("Enter date (YYYY-MM-DD)")
                .default(Local::now().format("%Y-%m-%d").to_string())
                .validate_with(|input: &String| -> Result<(), &str> {
                    NaiveDate::parse_from_str(input, "%Y-%m-%d")
                        .map(|_| ())
                        .map_err(|_| "Invalid date format. Use YYYY-MM-DD")
                })
                .interact()?;
            
            let mut params = HashMap::new();
            params.insert("date".to_string(), date);
            client.get("/api/apod", params).await?
        }
        2 => {
            // Random pictures
            let count: String = Input::new()
                .with_prompt("How many random pictures? (1-10)")
                .default("5".to_string())
                .validate_with(|input: &String| -> Result<(), &str> {
                    match input.parse::<u32>() {
                        Ok(n) if n >= 1 && n <= 10 => Ok(()),
                        _ => Err("Please enter a number between 1 and 10")
                    }
                })
                .interact()?;
            
            let mut params = HashMap::new();
            params.insert("count".to_string(), count);
            client.get("/api/apod", params).await?
        }
        3 => {
            // Date range
            let start_date: String = Input::new()
                .with_prompt("Enter start date (YYYY-MM-DD)")
                .validate_with(validate_date)
                .interact()?;
            
            let end_date: String = Input::new()
                .with_prompt("Enter end date (YYYY-MM-DD)")
                .validate_with(validate_date)
                .interact()?;
            
            let mut params = HashMap::new();
            params.insert("start_date".to_string(), start_date);
            params.insert("end_date".to_string(), end_date);
            client.get("/api/apod", params).await?
        }
        _ => unreachable!(),
    };
    
    display_results(data, output_format)?;
    Ok(())
}

async fn explore_asteroids(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "☄️ Near Earth Objects".bright_yellow().bold());
    
    let options = vec![
        "Asteroid Feed (This Week)",
        "Asteroid Feed (Custom Dates)",
        "Lookup Specific Asteroid",
        "Browse All Asteroids",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to explore?")
        .items(&options)
        .interact()?;
    
    let data = match selection {
        0 => {
            // This week's feed
            client.get("/api/neo/feed", HashMap::new()).await?
        }
        1 => {
            // Custom date range
            let start_date: String = Input::new()
                .with_prompt("Enter start date (YYYY-MM-DD)")
                .validate_with(validate_date)
                .interact()?;
            
            let end_date: String = Input::new()
                .with_prompt("Enter end date (YYYY-MM-DD, max 7 days)")
                .validate_with(validate_date)
                .interact()?;
            
            let mut params = HashMap::new();
            params.insert("start_date".to_string(), start_date);
            params.insert("end_date".to_string(), end_date);
            client.get("/api/neo/feed", params).await?
        }
        2 => {
            // Lookup specific asteroid
            let asteroid_id: String = Input::new()
                .with_prompt("Enter asteroid ID (e.g., 3542519)")
                .interact()?;
            
            client.get(&format!("/api/neo/{}", asteroid_id), HashMap::new()).await?
        }
        3 => {
            // Browse asteroids
            let page: String = Input::new()
                .with_prompt("Page number")
                .default("0".to_string())
                .interact()?;
            
            let size: String = Input::new()
                .with_prompt("Results per page")
                .default("20".to_string())
                .interact()?;
            
            let mut params = HashMap::new();
            params.insert("page".to_string(), page);
            params.insert("size".to_string(), size);
            client.get("/api/neo/browse", params).await?
        }
        _ => unreachable!(),
    };
    
    display_results(data, output_format)?;
    Ok(())
}

async fn explore_space_weather(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "🌞 Space Weather (DONKI)".bright_yellow().bold());
    
    let options = vec![
        "Coronal Mass Ejections (CME)",
        "Solar Flares",
        "Geomagnetic Storms",
        "Solar Wind Predictions",
        "All Notifications",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select space weather data")
        .items(&options)
        .interact()?;
    
    let use_dates = Confirm::new()
        .with_prompt("Specify date range? (default: last 30 days)")
        .default(false)
        .interact()?;
    
    let mut params = HashMap::new();
    
    if use_dates {
        let start_date: String = Input::new()
            .with_prompt("Enter start date (YYYY-MM-DD)")
            .validate_with(validate_date)
            .interact()?;
        params.insert("startDate".to_string(), start_date);
        
        let end_date: String = Input::new()
            .with_prompt("Enter end date (YYYY-MM-DD)")
            .validate_with(validate_date)
            .interact()?;
        params.insert("endDate".to_string(), end_date);
    }
    
    let endpoint = match selection {
        0 => "/api/donki/cme",
        1 => "/api/donki/flr",
        2 => "/api/donki/gst",
        3 => "/api/donki/wsa",
        4 => "/api/donki/notifications",
        _ => unreachable!(),
    };
    
    let data = client.get(endpoint, params).await?;
    display_results(data, output_format)?;
    Ok(())
}

async fn explore_mars(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "🔴 Mars Rover Photos".bright_yellow().bold());
    
    let rovers = vec!["Curiosity", "Opportunity", "Spirit"];
    let rover_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Mars rover")
        .items(&rovers)
        .default(0)
        .interact()?;
    
    let rover = rovers[rover_idx].to_lowercase();
    
    let options = vec![
        "Latest Photos",
        "Photos by Sol (Martian Day)",
        "Photos by Earth Date",
        "Mission Manifest",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to see?")
        .items(&options)
        .interact()?;
    
    let data = match selection {
        0 => {
            // Latest photos
            client.get(&format!("/api/mars-photos/{}/latest", rover), HashMap::new()).await?
        }
        1 => {
            // Photos by sol
            let sol: String = Input::new()
                .with_prompt("Enter sol (Martian day)")
                .default("1000".to_string())
                .interact()?;
            
            let mut params = HashMap::new();
            params.insert("sol".to_string(), sol);
            
            // Ask for camera selection
            let cameras = get_rover_cameras(&rover);
            let camera_selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Select cameras (space to select, enter to confirm)")
                .items(&cameras)
                .interact()?;
            
            if !camera_selections.is_empty() {
                let selected_camera = &cameras[camera_selections[0]];
                params.insert("camera".to_string(), selected_camera.to_string());
            }
            
            client.get(&format!("/api/mars-photos/{}/photos", rover), params).await?
        }
        2 => {
            // Photos by Earth date
            let date: String = Input::new()
                .with_prompt("Enter Earth date (YYYY-MM-DD)")
                .validate_with(validate_date)
                .interact()?;
            
            let mut params = HashMap::new();
            params.insert("earth_date".to_string(), date);
            
            client.get(&format!("/api/mars-photos/{}/photos", rover), params).await?
        }
        3 => {
            // Mission manifest
            client.get(&format!("/api/mars-photos/manifests/{}", rover), HashMap::new()).await?
        }
        _ => unreachable!(),
    };
    
    display_results(data, output_format)?;
    Ok(())
}

async fn explore_earth(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "🌍 Earth Satellite Imagery".bright_yellow().bold());
    
    let lat: String = Input::new()
        .with_prompt("Enter latitude (-90 to 90)")
        .default("29.78".to_string())
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<f64>() {
                Ok(lat) if lat >= -90.0 && lat <= 90.0 => Ok(()),
                _ => Err("Latitude must be between -90 and 90")
            }
        })
        .interact()?;
    
    let lon: String = Input::new()
        .with_prompt("Enter longitude (-180 to 180)")
        .default("-95.33".to_string())
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<f64>() {
                Ok(lon) if lon >= -180.0 && lon <= 180.0 => Ok(()),
                _ => Err("Longitude must be between -180 and 180")
            }
        })
        .interact()?;
    
    let date: String = Input::new()
        .with_prompt("Enter date (YYYY-MM-DD)")
        .default("2023-01-01".to_string())
        .validate_with(validate_date)
        .interact()?;
    
    let options = vec!["Get Satellite Image", "Check Available Asset Dates"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&options)
        .interact()?;
    
    let mut params = HashMap::new();
    params.insert("lat".to_string(), lat);
    params.insert("lon".to_string(), lon);
    params.insert("date".to_string(), date);
    
    match selection {
        0 => {
            println!("\n{}", "Note: This will download a satellite image.".bright_yellow());
            if Confirm::new().with_prompt("Continue?").default(true).interact()? {
                // For image endpoint, we'll just show the URL
                println!("\n{}", "Image URL:".bright_green());
                let url = format!("{}/api/earth/imagery?lat={}&lon={}&date={}&api_key={}",
                    client.base_url(),
                    params.get("lat").unwrap(),
                    params.get("lon").unwrap(),
                    params.get("date").unwrap(),
                    client.api_key().unwrap_or("DEMO_KEY")
                );
                println!("{}", url);
            }
        }
        1 => {
            let data = client.get("/api/earth/assets", params).await?;
            display_results(data, output_format)?;
        }
        _ => unreachable!(),
    }
    
    Ok(())
}

async fn explore_epic(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "🌏 EPIC - Earth from Space".bright_yellow().bold());
    
    let image_types = vec!["Natural Color", "Enhanced Color"];
    let type_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select image type")
        .items(&image_types)
        .default(0)
        .interact()?;
    
    let image_type = if type_idx == 0 { "natural" } else { "enhanced" };
    
    let options = vec!["All Available Dates", "Specific Date"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to see?")
        .items(&options)
        .interact()?;
    
    let endpoint = match selection {
        0 => format!("/api/epic/{}/all", image_type),
        1 => {
            let date: String = Input::new()
                .with_prompt("Enter date (YYYY-MM-DD)")
                .validate_with(validate_date)
                .interact()?;
            format!("/api/epic/{}/date/{}", image_type, date)
        }
        _ => unreachable!(),
    };
    
    let data = client.get(&endpoint, HashMap::new()).await?;
    display_results(data, output_format)?;
    Ok(())
}

async fn explore_tech(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "💡 NASA Technology Transfer".bright_yellow().bold());
    
    let options = vec!["Patents", "Software", "Spinoff Technologies"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select technology type")
        .items(&options)
        .interact()?;
    
    let endpoint = match selection {
        0 => "/api/techtransfer/patents",
        1 => "/api/techtransfer/software",
        2 => "/api/techtransfer/spinoffs",
        _ => unreachable!(),
    };
    
    let search = Confirm::new()
        .with_prompt("Search with keywords?")
        .default(false)
        .interact()?;
    
    let mut params = HashMap::new();
    if search {
        let query: String = Input::new()
            .with_prompt("Enter search keywords")
            .interact()?;
        params.insert("query".to_string(), query);
    }
    
    let data = client.get(endpoint, params).await?;
    display_results(data, output_format)?;
    Ok(())
}

async fn explore_media(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "🖼️ NASA Image and Video Library".bright_yellow().bold());
    
    let query: String = Input::new()
        .with_prompt("Enter search query")
        .default("apollo".to_string())
        .interact()?;
    
    let mut params = HashMap::new();
    params.insert("q".to_string(), query);
    
    // Advanced options
    if Confirm::new()
        .with_prompt("Add advanced search filters?")
        .default(false)
        .interact()? {
        
        // Media type
        let media_types = vec!["Any", "Image", "Video", "Audio"];
        let media_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select media type")
            .items(&media_types)
            .default(0)
            .interact()?;
        
        if media_idx > 0 {
            params.insert("media_type".to_string(), media_types[media_idx].to_lowercase());
        }
        
        // NASA Center
        let centers = vec!["Any", "KSC", "JSC", "JPL", "GSFC", "ARC", "HQ"];
        let center_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select NASA center")
            .items(&centers)
            .default(0)
            .interact()?;
        
        if center_idx > 0 {
            params.insert("center".to_string(), centers[center_idx].to_string());
        }
    }
    
    let data = client.get("/api/media/search", params).await?;
    display_results(data, output_format)?;
    Ok(())
}

async fn explore_exoplanets(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "🪐 Exoplanet Archive".bright_yellow().bold());
    
    let options = vec![
        "Search Kepler Discoveries",
        "Custom SQL Query",
        "Browse Recent Discoveries",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to explore?")
        .items(&options)
        .interact()?;
    
    let mut params = HashMap::new();
    
    let query = match selection {
        0 => {
            // Kepler search
            let name_filter = Input::<String>::new()
                .with_prompt("Filter by planet name (optional)")
                .allow_empty(true)
                .interact()?;
            
            if name_filter.is_empty() {
                "select * from ps where pl_name like 'Kepler%' limit 20".to_string()
            } else {
                format!("select * from ps where pl_name like 'Kepler%{}%' limit 20", name_filter)
            }
        }
        1 => {
            // Custom query
            println!("\n{}", "Example queries:".bright_cyan());
            println!("  - select * from ps where pl_masse < 10");
            println!("  - select pl_name, pl_masse, pl_rade from ps where sy_snum > 1");
            
            Input::new()
                .with_prompt("Enter ADQL query")
                .interact()?
        }
        2 => {
            // Recent discoveries
            "select * from ps order by rowupdate desc limit 20".to_string()
        }
        _ => unreachable!(),
    };
    
    params.insert("query".to_string(), query);
    params.insert("format".to_string(), "json".to_string());
    
    let data = client.get("/api/exoplanets/query", params).await?;
    display_results(data, output_format)?;
    Ok(())
}

async fn explore_ssd(client: &ApiClient, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "☄️ Solar System Dynamics".bright_yellow().bold());
    
    let options = vec![
        "Close Approach Data",
        "Fireball Atmospheric Impacts",
        "Sentry Impact Risk",
        "Small Body Database Lookup",
        "NHATS - Human Accessible Targets",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select data type")
        .items(&options)
        .interact()?;
    
    let data = match selection {
        0 => {
            // Close approach
            let mut params = HashMap::new();
            
            if Confirm::new()
                .with_prompt("Filter by date range?")
                .default(true)
                .interact()? {
                
                let date_min: String = Input::new()
                    .with_prompt("Enter start date (YYYY-MM-DD)")
                    .validate_with(validate_date)
                    .interact()?;
                params.insert("date-min".to_string(), date_min);
                
                let date_max: String = Input::new()
                    .with_prompt("Enter end date (YYYY-MM-DD)")
                    .validate_with(validate_date)
                    .interact()?;
                params.insert("date-max".to_string(), date_max);
            }
            
            if Confirm::new()
                .with_prompt("Only show potentially hazardous asteroids?")
                .default(false)
                .interact()? {
                params.insert("pha".to_string(), "true".to_string());
            }
            
            client.get("/api/ssd/cad", params).await?
        }
        1 => {
            // Fireballs
            let mut params = HashMap::new();
            
            if Confirm::new()
                .with_prompt("Require location data?")
                .default(true)
                .interact()? {
                params.insert("req-loc".to_string(), "true".to_string());
            }
            
            client.get("/api/ssd/fireballs", params).await?
        }
        2 => {
            // Sentry
            let mut params = HashMap::new();
            
            if Confirm::new()
                .with_prompt("Search for specific object?")
                .default(false)
                .interact()? {
                let object: String = Input::new()
                    .with_prompt("Enter object designation (e.g., 99942 Apophis)")
                    .interact()?;
                params.insert("des".to_string(), object);
            }
            
            client.get("/api/ssd/sentry", params).await?
        }
        3 => {
            // SBDB lookup
            let object: String = Input::new()
                .with_prompt("Enter object name or designation (e.g., 433 Eros)")
                .interact()?;
            
            let mut params = HashMap::new();
            params.insert("sstr".to_string(), object);
            
            client.get("/api/ssd/sbdb", params).await?
        }
        4 => {
            // NHATS
            client.get("/api/ssd/nhats", HashMap::new()).await?
        }
        _ => unreachable!(),
    };
    
    display_results(data, output_format)?;
    Ok(())
}

async fn configure_settings(context: &CommandContext) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{}", "⚙️ Settings".bright_yellow().bold());
    
    let options = vec![
        "Change Output Format",
        "Toggle Cache",
        "Clear Cache",
        "Show Current Configuration",
        "Back to Main Menu",
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select setting to change")
        .items(&options)
        .interact()?;
    
    match selection {
        0 => {
            // Change output format
            let formats = vec!["Pretty", "JSON", "Table", "CSV"];
            let format_idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select output format")
                .items(&formats)
                .default(0)
                .interact()?;
            
            let format = match format_idx {
                0 => "pretty",
                1 => "json",
                2 => "table",
                3 => "csv",
                _ => unreachable!(),
            };
            
            context.config_manager.set("output_format", format).await?;
            println!("{}", format!("Output format set to: {}", format).bright_green());
        }
        1 => {
            // Toggle cache
            let current = context.config.use_cache;
            let new_value = !current;
            context.config_manager.set("use_cache", &new_value.to_string()).await?;
            println!("{}", format!("Cache {}", if new_value { "enabled" } else { "disabled" }).bright_green());
        }
        2 => {
            // Clear cache - TODO: implement cache clearing
            println!("{}", "Cache clearing not yet implemented".yellow());
        }
        3 => {
            // Show configuration
            println!("\n{}", "Current Configuration:".bright_cyan());
            println!("API Endpoint: {}", context.config.api_endpoint);
            println!("Output Format: {:?}", context.config.output_format);
            println!("Cache Enabled: {}", context.config.use_cache);
            println!("Cache TTL: {} minutes", context.config.cache_ttl_minutes);
            println!("Cache Directory: {}", context.config.cache_dir.display());
        }
        4 => {
            // Back to main menu
            return Ok(());
        }
        _ => unreachable!(),
    }
    
    Ok(())
}

fn display_results(data: Value, output_format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    let formatter = crate::cli::output::create_formatter(*output_format);
    let formatted = formatter.format(&data)?;
    
    // For long output, use a pager-like display
    let lines: Vec<&str> = formatted.lines().collect();
    if lines.len() > 30 {
        println!("\n{}", format!("Showing {} lines of output:", lines.len()).bright_cyan());
        println!("{}", "=".repeat(50).bright_cyan());
        
        for (i, line) in lines.iter().enumerate() {
            println!("{}", line);
            
            // Pause every 20 lines
            if (i + 1) % 20 == 0 && i + 1 < lines.len() {
                if !Confirm::new()
                    .with_prompt("Continue?")
                    .default(true)
                    .interact()? {
                    break;
                }
            }
        }
    } else {
        println!("\n{}", formatted);
    }
    
    Ok(())
}

fn get_rover_cameras(rover: &str) -> Vec<&'static str> {
    match rover {
        "curiosity" => vec!["ALL", "FHAZ", "RHAZ", "MAST", "CHEMCAM", "MAHLI", "MARDI", "NAVCAM"],
        "opportunity" | "spirit" => vec!["ALL", "FHAZ", "RHAZ", "NAVCAM", "PANCAM", "MINITES"],
        _ => vec!["ALL"],
    }
}

fn validate_date(input: &String) -> Result<(), &'static str> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .map(|_| ())
        .map_err(|_| "Invalid date format. Use YYYY-MM-DD")
}