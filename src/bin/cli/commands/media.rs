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
            let mut params = HashMap::new();
            
            if let Some(query) = sub_matches.get_one::<String>("query") {
                params.insert("q".to_string(), query.clone());
            }
            if let Some(media_type) = sub_matches.get_one::<String>("media-type") {
                params.insert("media_type".to_string(), media_type.clone());
            }
            if let Some(center) = sub_matches.get_one::<String>("center") {
                params.insert("center".to_string(), center.clone());
            }
            if let Some(year_start) = sub_matches.get_one::<String>("year-start") {
                params.insert("year_start".to_string(), year_start.clone());
            }
            if let Some(year_end) = sub_matches.get_one::<String>("year-end") {
                params.insert("year_end".to_string(), year_end.clone());
            }
            if let Some(page) = sub_matches.get_one::<String>("page") {
                params.insert("page".to_string(), page.clone());
            }
            
            let data = client.get("/api/media/search", params).await?;
            Ok(Some(data))
        }
        Some(("asset", sub_matches)) => {
            let nasa_id = sub_matches.get_one::<String>("nasa-id").unwrap();
            let params = HashMap::new();
            let data = client.get(&format!("/api/media/asset/{}", nasa_id), params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: search or asset");
            Ok(None)
        }
    }
}