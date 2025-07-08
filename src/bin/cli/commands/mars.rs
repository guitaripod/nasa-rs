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
        Some(("photos", sub_matches)) => {
            let rover = sub_matches.get_one::<String>("rover").unwrap();
            let mut params = HashMap::new();
            
            // Either sol or earth_date is required
            if let Some(sol) = sub_matches.get_one::<String>("sol") {
                params.insert("sol".to_string(), sol.clone());
            } else if let Some(earth_date) = sub_matches.get_one::<String>("earth-date") {
                params.insert("earth_date".to_string(), earth_date.clone());
            } else {
                return Err("Either --sol or --earth-date is required".into());
            }
            
            if let Some(camera) = sub_matches.get_one::<String>("camera") {
                params.insert("camera".to_string(), camera.clone());
            }
            
            if let Some(page) = sub_matches.get_one::<String>("page") {
                params.insert("page".to_string(), page.clone());
            }
            
            let data = client.get(&format!("/api/mars-photos/{rover}/photos"), params).await?;
            Ok(Some(data))
        }
        Some(("latest", sub_matches)) => {
            let rover = sub_matches.get_one::<String>("rover").unwrap();
            let params = HashMap::new();
            
            let data = client.get(&format!("/api/mars-photos/{rover}/latest"), params).await?;
            Ok(Some(data))
        }
        Some(("manifest", sub_matches)) => {
            let rover = sub_matches.get_one::<String>("rover").unwrap();
            let params = HashMap::new();
            
            let data = client.get(&format!("/api/mars-photos/manifests/{rover}"), params).await?;
            Ok(Some(data))
        }
        Some(("batch", sub_matches)) => {
            let rover = sub_matches.get_one::<String>("rover").unwrap();
            let limit = sub_matches.get_one::<String>("limit").unwrap().parse::<usize>()?;
            
            let mut all_photos = Vec::new();
            
            // Determine if we're using sol range or date range
            if let (Some(sol_start), Some(sol_end)) = (
                sub_matches.get_one::<String>("sol-start"),
                sub_matches.get_one::<String>("sol-end")
            ) {
                let start = sol_start.parse::<u32>()?;
                let end = sol_end.parse::<u32>()?;
                
                for sol in start..=end {
                    if all_photos.len() >= limit {
                        break;
                    }
                    
                    let mut params = HashMap::new();
                    params.insert("sol".to_string(), sol.to_string());
                    
                    if let Some(cameras) = sub_matches.get_one::<String>("cameras") {
                        // For each camera, make a separate request
                        for camera in cameras.split(',') {
                            let mut cam_params = params.clone();
                            cam_params.insert("camera".to_string(), camera.trim().to_string());
                            
                            if let Ok(data) = client.get(&format!("/api/mars-photos/{rover}/photos"), cam_params).await {
                                if let Some(photos) = data.get("photos").and_then(|p| p.as_array()) {
                                    all_photos.extend(photos.clone());
                                }
                            }
                        }
                    } else if let Ok(data) = client.get(&format!("/api/mars-photos/{rover}/photos"), params).await {
                        if let Some(photos) = data.get("photos").and_then(|p| p.as_array()) {
                            all_photos.extend(photos.clone());
                        }
                    }
                }
            } else if let (Some(_date_start), Some(_date_end)) = (
                sub_matches.get_one::<String>("date-start"),
                sub_matches.get_one::<String>("date-end")
            ) {
                // TODO: Implement date range iteration
                return Err("Date range batch not yet implemented. Use sol range instead.".into());
            } else {
                return Err("Either --sol-start/--sol-end or --date-start/--date-end required".into());
            }
            
            // Limit the results
            all_photos.truncate(limit);
            
            Ok(Some(json!({
                "count": all_photos.len(),
                "photos": all_photos
            })))
        }
        Some(("collection", sub_matches)) => {
            let rover = sub_matches.get_one::<String>("rover").unwrap();
            let sol = sub_matches.get_one::<String>("sol").unwrap();
            let all_pages = sub_matches.get_flag("all-pages");
            
            let mut all_photos = Vec::new();
            let mut page = 1;
            
            loop {
                let mut params = HashMap::new();
                params.insert("sol".to_string(), sol.clone());
                params.insert("page".to_string(), page.to_string());
                
                if let Some(cameras) = sub_matches.get_one::<String>("cameras") {
                    // Get photos from multiple cameras
                    for camera in cameras.split(',') {
                        let mut cam_params = params.clone();
                        cam_params.insert("camera".to_string(), camera.trim().to_string());
                        
                        if let Ok(data) = client.get(&format!("/api/mars-photos/{rover}/photos"), cam_params).await {
                            if let Some(photos) = data.get("photos").and_then(|p| p.as_array()) {
                                if photos.is_empty() {
                                    continue;
                                }
                                all_photos.extend(photos.clone());
                            }
                        }
                    }
                } else if let Ok(data) = client.get(&format!("/api/mars-photos/{rover}/photos"), params).await {
                    if let Some(photos) = data.get("photos").and_then(|p| p.as_array()) {
                        if photos.is_empty() {
                            break;
                        }
                        all_photos.extend(photos.clone());
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                
                if !all_pages {
                    break;
                }
                
                page += 1;
            }
            
            Ok(Some(json!({
                "count": all_photos.len(),
                "sol": sol,
                "rover": rover,
                "photos": all_photos
            })))
        }
        Some(("download", sub_matches)) => {
            let rover = sub_matches.get_one::<String>("rover").unwrap();
            let sol = sub_matches.get_one::<String>("sol").unwrap();
            let save_dir = sub_matches.get_one::<String>("save-to").unwrap();
            
            // Create directory if it doesn't exist
            fs::create_dir_all(save_dir)?;
            
            let mut params = HashMap::new();
            params.insert("sol".to_string(), sol.clone());
            
            if let Some(camera) = sub_matches.get_one::<String>("camera") {
                params.insert("camera".to_string(), camera.clone());
            }
            
            let data = client.get(&format!("/api/mars-photos/{rover}/photos"), params).await?;
            
            if let Some(photos) = data.get("photos").and_then(|p| p.as_array()) {
                let mut downloaded = 0;
                
                for photo in photos {
                    if let Some(img_src) = photo.get("img_src").and_then(|s| s.as_str()) {
                        if let Some(id) = photo.get("id") {
                            let filename = format!("{rover}_sol{sol}_{id}.jpg");
                            let filepath = Path::new(save_dir).join(&filename);
                            
                            // TODO: Actually download the image
                            // For now, we'll just report what would be downloaded
                            println!("Would download: {} -> {}", img_src, filepath.display());
                            downloaded += 1;
                        }
                    }
                }
                
                Ok(Some(json!({
                    "status": "download_planned",
                    "count": downloaded,
                    "directory": save_dir
                })))
            } else {
                Ok(Some(json!({
                    "status": "no_photos_found"
                })))
            }
        }
        _ => {
            eprintln!("Please specify a subcommand: photos, latest, manifest, batch, collection, or download");
            Ok(None)
        }
    }
}