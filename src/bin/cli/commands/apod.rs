use clap::ArgMatches;
use serde_json::Value;
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
        _ => {
            eprintln!("Please specify a subcommand: today, date, or random");
            Ok(None)
        }
    }
}