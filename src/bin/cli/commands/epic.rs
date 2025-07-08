use clap::ArgMatches;
use serde_json::{Value, json};
use std::collections::HashMap;
use crate::cli::api::ApiClient;
use chrono::{NaiveDate, Duration};
use std::fs;
use std::path::Path;

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
        Some(("bulk", sub_matches)) => {
            let image_type = sub_matches.get_one::<String>("type").unwrap();
            let start_date = sub_matches.get_one::<String>("start-date").unwrap();
            let end_date = sub_matches.get_one::<String>("end-date").unwrap();
            let limit_per_day = sub_matches.get_one::<String>("limit").map(|s| s.parse::<usize>().unwrap_or(50));
            
            // Parse dates
            let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
            let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")?;
            
            let mut all_images = Vec::new();
            let mut current_date = start;
            
            while current_date <= end {
                let date_str = current_date.format("%Y-%m-%d").to_string();
                let path = format!("/api/epic/{image_type}/date/{date_str}");
                let params = HashMap::new();
                
                if let Ok(data) = client.get(&path, params).await {
                    if let Some(images) = data.as_array() {
                        let mut day_images = images.clone();
                        if let Some(limit) = limit_per_day {
                            day_images.truncate(limit);
                        }
                        all_images.extend(day_images);
                    }
                }
                
                current_date += Duration::days(1);
            }
            
            Ok(Some(json!({
                "count": all_images.len(),
                "start_date": start_date,
                "end_date": end_date,
                "type": image_type,
                "images": all_images
            })))
        }
        Some(("archive", sub_matches)) => {
            let image_type = sub_matches.get_one::<String>("type").unwrap();
            let year = sub_matches.get_one::<String>("year").unwrap().parse::<i32>()?;
            let month = sub_matches.get_one::<String>("month").unwrap().parse::<u32>()?;
            
            let mut all_images = Vec::new();
            
            // Calculate days in month
            let days_in_month = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
                _ => return Err("Invalid month".into()),
            };
            
            for day in 1..=days_in_month {
                let date_str = format!("{year:04}-{month:02}-{day:02}");
                let path = format!("/api/epic/{image_type}/date/{date_str}");
                let params = HashMap::new();
                
                if let Ok(data) = client.get(&path, params).await {
                    if let Some(images) = data.as_array() {
                        all_images.extend(images.clone());
                    }
                }
            }
            
            Ok(Some(json!({
                "count": all_images.len(),
                "year": year,
                "month": month,
                "type": image_type,
                "images": all_images
            })))
        }
        Some(("download-set", sub_matches)) => {
            let image_type = sub_matches.get_one::<String>("type").unwrap();
            let date = sub_matches.get_one::<String>("date").unwrap();
            let save_dir = sub_matches.get_one::<String>("save-to").unwrap();
            let format = sub_matches.get_one::<String>("format").unwrap();
            
            // Create directory if it doesn't exist
            fs::create_dir_all(save_dir)?;
            
            let path = format!("/api/epic/{image_type}/date/{date}");
            let params = HashMap::new();
            let data = client.get(&path, params).await?;
            
            if let Some(images) = data.as_array() {
                let mut downloaded = 0;
                
                for image in images {
                    if let (Some(identifier), Some(_caption)) = (
                        image.get("identifier").and_then(|i| i.as_str()),
                        image.get("caption").and_then(|c| c.as_str())
                    ) {
                        let filename = format!("{identifier}_{image_type}.{format}");
                        let filepath = Path::new(save_dir).join(&filename);
                        
                        // TODO: Actually download the image
                        // For now, we'll just report what would be downloaded
                        println!("Would download: {} -> {}", identifier, filepath.display());
                        downloaded += 1;
                    }
                }
                
                Ok(Some(json!({
                    "status": "download_planned",
                    "count": downloaded,
                    "date": date,
                    "type": image_type,
                    "directory": save_dir
                })))
            } else {
                Ok(Some(json!({
                    "status": "no_images_found",
                    "date": date
                })))
            }
        }
        _ => {
            eprintln!("Please specify a subcommand: natural, enhanced, bulk, archive, or download-set");
            Ok(None)
        }
    }
}