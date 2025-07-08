use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::api::ApiClient;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("natural", sub_matches)) => {
            let date = sub_matches.get_one::<String>("date");
            let path = if let Some(d) = date {
                if d == "all" {
                    "/api/epic/natural/all".to_string()
                } else {
                    format!("/api/epic/natural/date/{d}")
                }
            } else {
                "/api/epic/natural/all".to_string()
            };
            
            let params = HashMap::new();
            let data = client.get(&path, params).await?;
            Ok(Some(data))
        }
        Some(("enhanced", sub_matches)) => {
            let date = sub_matches.get_one::<String>("date");
            let path = if let Some(d) = date {
                if d == "all" {
                    "/api/epic/enhanced/all".to_string()
                } else {
                    format!("/api/epic/enhanced/date/{d}")
                }
            } else {
                "/api/epic/enhanced/all".to_string()
            };
            
            let params = HashMap::new();
            let data = client.get(&path, params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: natural or enhanced");
            Ok(None)
        }
    }
}