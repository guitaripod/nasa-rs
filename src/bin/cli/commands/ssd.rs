use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::api::ApiClient;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("close-approach", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(date_min) = sub_matches.get_one::<String>("date-min") {
                params.insert("date-min".to_string(), date_min.clone());
            }
            if let Some(date_max) = sub_matches.get_one::<String>("date-max") {
                params.insert("date-max".to_string(), date_max.clone());
            }
            if let Some(dist_max) = sub_matches.get_one::<String>("dist-max") {
                params.insert("dist-max".to_string(), dist_max.clone());
            }
            if sub_matches.get_flag("pha") {
                params.insert("pha".to_string(), "true".to_string());
            }
            
            let data = client.get("/api/ssd/cad", params).await?;
            Ok(Some(data))
        }
        Some(("fireballs", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(date_min) = sub_matches.get_one::<String>("date-min") {
                params.insert("date-min".to_string(), date_min.clone());
            }
            if let Some(date_max) = sub_matches.get_one::<String>("date-max") {
                params.insert("date-max".to_string(), date_max.clone());
            }
            if sub_matches.get_flag("req-loc") {
                params.insert("req-loc".to_string(), "true".to_string());
            }
            
            let data = client.get("/api/ssd/fireballs", params).await?;
            Ok(Some(data))
        }
        Some(("sentry", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(object) = sub_matches.get_one::<String>("object") {
                params.insert("des".to_string(), object.clone());
            }
            
            let data = client.get("/api/ssd/sentry", params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: close-approach, fireballs, or sentry");
            Ok(None)
        }
    }
}