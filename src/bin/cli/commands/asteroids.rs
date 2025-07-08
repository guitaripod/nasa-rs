use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::api::ApiClient;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("feed", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(start_date) = sub_matches.get_one::<String>("start-date") {
                params.insert("start_date".to_string(), start_date.clone());
            }
            
            if let Some(end_date) = sub_matches.get_one::<String>("end-date") {
                params.insert("end_date".to_string(), end_date.clone());
            }
            
            let data = client.get("/api/neo/feed", params).await?;
            Ok(Some(data))
        }
        Some(("lookup", sub_matches)) => {
            let asteroid_id = sub_matches.get_one::<String>("id").unwrap();
            let params = HashMap::new();
            
            let data = client.get(&format!("/api/neo/{asteroid_id}"), params).await?;
            Ok(Some(data))
        }
        Some(("browse", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(page) = sub_matches.get_one::<String>("page") {
                params.insert("page".to_string(), page.clone());
            }
            
            if let Some(size) = sub_matches.get_one::<String>("size") {
                params.insert("size".to_string(), size.clone());
            }
            
            let data = client.get("/api/neo/browse", params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: feed, lookup, or browse");
            Ok(None)
        }
    }
}