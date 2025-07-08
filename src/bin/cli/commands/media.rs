use clap::ArgMatches;
use serde_json::{Value, json};
use std::collections::HashMap;
use crate::cli::api::ApiClient;
use std::fs;
use std::path::Path;

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
            let data = client.get(&format!("/api/media/asset/{nasa_id}"), params).await?;
            Ok(Some(data))
        }
        Some(("collection", sub_matches)) => {
            let query = sub_matches.get_one::<String>("query").unwrap();
            let all_pages = sub_matches.get_flag("all-pages");
            let limit = sub_matches.get_one::<String>("limit").unwrap().parse::<usize>()?;
            
            let mut all_items = Vec::new();
            let mut page = 1;
            
            loop {
                let mut params = HashMap::new();
                params.insert("q".to_string(), query.clone());
                params.insert("page".to_string(), page.to_string());
                
                if let Some(media_type) = sub_matches.get_one::<String>("media-type") {
                    params.insert("media_type".to_string(), media_type.clone());
                }
                
                let data = client.get("/api/media/search", params).await?;
                
                if let Some(collection) = data.get("collection") {
                    if let Some(items) = collection.get("items").and_then(|i| i.as_array()) {
                        if items.is_empty() {
                            break;
                        }
                        
                        for item in items {
                            if all_items.len() >= limit {
                                break;
                            }
                            all_items.push(item.clone());
                        }
                        
                        if !all_pages || all_items.len() >= limit {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                
                page += 1;
            }
            
            Ok(Some(json!({
                "query": query,
                "total_items": all_items.len(),
                "collection": {
                    "items": all_items
                }
            })))
        }
        Some(("batch-assets", sub_matches)) => {
            let nasa_ids: Vec<&str> = sub_matches.get_many::<String>("nasa-ids")
                .unwrap()
                .map(|s| s.as_str())
                .collect();
            
            let mut assets = Vec::new();
            
            for nasa_id in nasa_ids {
                let params = HashMap::new();
                if let Ok(data) = client.get(&format!("/api/media/asset/{nasa_id}"), params).await {
                    assets.push(json!({
                        "nasa_id": nasa_id,
                        "asset": data
                    }));
                } else {
                    assets.push(json!({
                        "nasa_id": nasa_id,
                        "error": "Failed to retrieve asset"
                    }));
                }
            }
            
            Ok(Some(json!({
                "count": assets.len(),
                "assets": assets
            })))
        }
        Some(("download-results", sub_matches)) => {
            let query = sub_matches.get_one::<String>("query").unwrap();
            let media_type = sub_matches.get_one::<String>("media-type").unwrap();
            let limit = sub_matches.get_one::<String>("limit").unwrap().parse::<usize>()?;
            let save_dir = sub_matches.get_one::<String>("save-to").unwrap();
            
            // Create directory if it doesn't exist
            fs::create_dir_all(save_dir)?;
            
            let mut params = HashMap::new();
            params.insert("q".to_string(), query.clone());
            params.insert("media_type".to_string(), media_type.clone());
            
            let data = client.get("/api/media/search", params).await?;
            
            let mut downloaded = 0;
            
            if let Some(collection) = data.get("collection") {
                if let Some(items) = collection.get("items").and_then(|i| i.as_array()) {
                    for (idx, item) in items.iter().enumerate() {
                        if idx >= limit {
                            break;
                        }
                        
                        if let Some(data_arr) = item.get("data").and_then(|d| d.as_array()) {
                            if let Some(first_data) = data_arr.first() {
                                if let (Some(nasa_id), Some(title)) = (
                                    first_data.get("nasa_id").and_then(|n| n.as_str()),
                                    first_data.get("title").and_then(|t| t.as_str())
                                ) {
                                    let safe_title = title.chars()
                                        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                                        .collect::<String>()
                                        .trim()
                                        .replace(' ', "_");
                                    
                                    let filename = format!("{}_{}.{}", nasa_id, safe_title, "jpg");
                                    let filepath = Path::new(save_dir).join(&filename);
                                    
                                    // TODO: Actually download the media
                                    println!("Would download: {} -> {}", nasa_id, filepath.display());
                                    downloaded += 1;
                                }
                            }
                        }
                    }
                }
            }
            
            Ok(Some(json!({
                "status": "download_planned",
                "query": query,
                "count": downloaded,
                "directory": save_dir
            })))
        }
        _ => {
            eprintln!("Please specify a subcommand: search, asset, collection, batch-assets, or download-results");
            Ok(None)
        }
    }
}