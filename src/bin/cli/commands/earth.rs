use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::api::ApiClient;
use std::fs;
use colored::Colorize;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("image", sub_matches)) => {
            let lat = sub_matches.get_one::<String>("lat").unwrap();
            let lon = sub_matches.get_one::<String>("lon").unwrap();
            
            let mut params = HashMap::new();
            params.insert("lat".to_string(), lat.clone());
            params.insert("lon".to_string(), lon.clone());
            
            if let Some(date) = sub_matches.get_one::<String>("date") {
                params.insert("date".to_string(), date.clone());
            }
            
            if let Some(dim) = sub_matches.get_one::<String>("dim") {
                params.insert("dim".to_string(), dim.clone());
            }
            
            // Earth imagery returns binary data
            let image_data = client.get_binary("/api/earth/imagery", params).await?;
            
            // Save the image
            let filename = format!("earth_{}_{}.png", lat, lon);
            fs::write(&filename, image_data)?;
            
            println!("{} Image saved to {}", "✓".green().bold(), filename.cyan());
            Ok(None)
        }
        Some(("assets", sub_matches)) => {
            let lat = sub_matches.get_one::<String>("lat").unwrap();
            let lon = sub_matches.get_one::<String>("lon").unwrap();
            let date = sub_matches.get_one::<String>("date").unwrap();
            
            let mut params = HashMap::new();
            params.insert("lat".to_string(), lat.clone());
            params.insert("lon".to_string(), lon.clone());
            params.insert("date".to_string(), date.clone());
            
            let data = client.get("/api/earth/assets", params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: image or assets");
            Ok(None)
        }
    }
}