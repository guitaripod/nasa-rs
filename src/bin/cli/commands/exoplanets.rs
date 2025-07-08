use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::api::ApiClient;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let query = sub_matches.get_one::<String>("query").unwrap();
            let format = sub_matches.get_one::<String>("format").unwrap();
            
            let mut params = HashMap::new();
            params.insert("query".to_string(), query.clone());
            params.insert("format".to_string(), format.clone());
            
            let data = client.get("/api/exoplanets/query", params).await?;
            Ok(Some(data))
        }
        Some(("kepler", sub_matches)) => {
            let mut query = "select * from ps where pl_name like 'Kepler%'".to_string();
            
            if let Some(name) = sub_matches.get_one::<String>("name") {
                query = format!("select * from ps where pl_name like 'Kepler%{name}%'");
            }
            
            let mut params = HashMap::new();
            params.insert("query".to_string(), query);
            params.insert("format".to_string(), "json".to_string());
            
            let data = client.get("/api/exoplanets/query", params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: search or kepler");
            Ok(None)
        }
    }
}