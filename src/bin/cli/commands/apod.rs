use clap::ArgMatches;
use serde_json::{Value, json};
use std::collections::HashMap;
use crate::cli::api::ApiClient;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("today", _)) => {
            let params = HashMap::new();
            let data = client.get("/api/apod", params).await?;
            Ok(Some(data))
        }
        Some(("date", sub_matches)) => {
            let date = sub_matches.get_one::<String>("date").unwrap();
            let mut params = HashMap::new();
            params.insert("date".to_string(), date.clone());
            
            let data = client.get("/api/apod", params).await?;
            Ok(Some(data))
        }
        Some(("random", sub_matches)) => {
            let count = sub_matches.get_one::<String>("count").unwrap();
            let mut params = HashMap::new();
            params.insert("count".to_string(), count.clone());
            
            let data = client.get("/api/apod", params).await?;
            Ok(Some(data))
        }
        Some(("range", sub_matches)) => {
            let start_date = sub_matches.get_one::<String>("start-date").unwrap();
            let end_date = sub_matches.get_one::<String>("end-date").unwrap();
            let mut params = HashMap::new();
            params.insert("start_date".to_string(), start_date.clone());
            params.insert("end_date".to_string(), end_date.clone());
            
            if sub_matches.get_flag("thumbs") {
                params.insert("thumbs".to_string(), "true".to_string());
            }
            
            let data = client.get("/api/apod", params).await?;
            Ok(Some(data))
        }
        Some(("batch", sub_matches)) => {
            let count = sub_matches.get_one::<String>("count").unwrap();
            let mut params = HashMap::new();
            
            // For batch, we'll use the random endpoint with count
            params.insert("count".to_string(), count.clone());
            
            if let Some(start_date) = sub_matches.get_one::<String>("start-date") {
                params.insert("start_date".to_string(), start_date.clone());
            }
            
            if sub_matches.get_flag("thumbs") {
                params.insert("thumbs".to_string(), "true".to_string());
            }
            
            let data = client.get("/api/apod", params).await?;
            
            // If the result is an array, wrap it for consistent output
            if data.is_array() {
                Ok(Some(json!({
                    "count": data.as_array().unwrap().len(),
                    "images": data
                })))
            } else {
                Ok(Some(data))
            }
        }
        _ => {
            eprintln!("Please specify a subcommand: today, date, random, range, or batch");
            Ok(None)
        }
    }
}