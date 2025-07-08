use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::api::ApiClient;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("cme", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(start_date) = sub_matches.get_one::<String>("start-date") {
                params.insert("startDate".to_string(), start_date.clone());
            }
            
            if let Some(end_date) = sub_matches.get_one::<String>("end-date") {
                params.insert("endDate".to_string(), end_date.clone());
            }
            
            let data = client.get("/api/donki/cme", params).await?;
            Ok(Some(data))
        }
        Some(("flare", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(start_date) = sub_matches.get_one::<String>("start-date") {
                params.insert("startDate".to_string(), start_date.clone());
            }
            
            if let Some(end_date) = sub_matches.get_one::<String>("end-date") {
                params.insert("endDate".to_string(), end_date.clone());
            }
            
            let data = client.get("/api/donki/flr", params).await?;
            Ok(Some(data))
        }
        Some(("storm", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(start_date) = sub_matches.get_one::<String>("start-date") {
                params.insert("startDate".to_string(), start_date.clone());
            }
            
            if let Some(end_date) = sub_matches.get_one::<String>("end-date") {
                params.insert("endDate".to_string(), end_date.clone());
            }
            
            let data = client.get("/api/donki/gst", params).await?;
            Ok(Some(data))
        }
        Some(("notifications", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(notification_type) = sub_matches.get_one::<String>("type") {
                params.insert("type".to_string(), notification_type.clone());
            }
            
            let data = client.get("/api/donki/notifications", params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: cme, flare, storm, or notifications");
            Ok(None)
        }
    }
}