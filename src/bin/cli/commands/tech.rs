use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::api::ApiClient;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("patents", sub_matches)) => {
            let mut params = HashMap::new();
            if let Some(query) = sub_matches.get_one::<String>("query") {
                params.insert("query".to_string(), query.clone());
            }
            let data = client.get("/api/techtransfer/patents", params).await?;
            Ok(Some(data))
        }
        Some(("software", sub_matches)) => {
            let mut params = HashMap::new();
            if let Some(query) = sub_matches.get_one::<String>("query") {
                params.insert("query".to_string(), query.clone());
            }
            let data = client.get("/api/techtransfer/software", params).await?;
            Ok(Some(data))
        }
        Some(("spinoffs", sub_matches)) => {
            let mut params = HashMap::new();
            if let Some(query) = sub_matches.get_one::<String>("query") {
                params.insert("query".to_string(), query.clone());
            }
            let data = client.get("/api/techtransfer/spinoffs", params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: patents, software, or spinoffs");
            Ok(None)
        }
    }
}