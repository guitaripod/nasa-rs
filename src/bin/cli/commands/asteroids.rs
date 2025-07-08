use clap::ArgMatches;
use serde_json::{Value, json};
use std::collections::HashMap;
use crate::cli::api::ApiClient;
use chrono::{Local, Duration, Datelike};

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
        Some(("feed-extended", sub_matches)) => {
            let mut params = HashMap::new();
            
            if let Some(start_date) = sub_matches.get_one::<String>("start-date") {
                params.insert("start_date".to_string(), start_date.clone());
            }
            if let Some(end_date) = sub_matches.get_one::<String>("end-date") {
                params.insert("end_date".to_string(), end_date.clone());
            }
            
            // Get the basic feed first
            let data = client.get("/api/neo/feed", params).await?;
            
            // Apply filters
            if let Some(neo_objects) = data.get("near_earth_objects").and_then(|n| n.as_object()) {
                let mut filtered_asteroids = Vec::new();
                
                for (_date, asteroids) in neo_objects {
                    if let Some(asteroid_list) = asteroids.as_array() {
                        for asteroid in asteroid_list {
                            let mut include = true;
                            
                            // Filter by hazardous status
                            if sub_matches.get_flag("hazardous") {
                                if let Some(hazardous) = asteroid.get("is_potentially_hazardous_asteroid").and_then(|h| h.as_bool()) {
                                    if !hazardous {
                                        include = false;
                                    }
                                }
                            }
                            
                            // Filter by size
                            if let Some(size_filter) = sub_matches.get_one::<String>("size") {
                                if let Some(diameter) = asteroid.get("estimated_diameter")
                                    .and_then(|d| d.get("kilometers"))
                                    .and_then(|k| k.get("estimated_diameter_max"))
                                    .and_then(|v| v.as_f64()) 
                                {
                                    let size_match = match size_filter.as_str() {
                                        "small" => diameter < 0.1,
                                        "medium" => (0.1..1.0).contains(&diameter),
                                        "large" => diameter >= 1.0,
                                        _ => true
                                    };
                                    if !size_match {
                                        include = false;
                                    }
                                }
                            }
                            
                            // Filter by distance
                            if let Some(min_dist) = sub_matches.get_one::<String>("min-distance") {
                                if let Some(approaches) = asteroid.get("close_approach_data").and_then(|c| c.as_array()) {
                                    if let Some(first_approach) = approaches.first() {
                                        if let Some(miss_distance) = first_approach.get("miss_distance")
                                            .and_then(|m| m.get("astronomical"))
                                            .and_then(|a| a.as_str())
                                            .and_then(|s| s.parse::<f64>().ok())
                                        {
                                            let min = min_dist.parse::<f64>().unwrap_or(0.0);
                                            if miss_distance < min {
                                                include = false;
                                            }
                                        }
                                    }
                                }
                            }
                            
                            if let Some(max_dist) = sub_matches.get_one::<String>("max-distance") {
                                if let Some(approaches) = asteroid.get("close_approach_data").and_then(|c| c.as_array()) {
                                    if let Some(first_approach) = approaches.first() {
                                        if let Some(miss_distance) = first_approach.get("miss_distance")
                                            .and_then(|m| m.get("astronomical"))
                                            .and_then(|a| a.as_str())
                                            .and_then(|s| s.parse::<f64>().ok())
                                        {
                                            let max = max_dist.parse::<f64>().unwrap_or(f64::MAX);
                                            if miss_distance > max {
                                                include = false;
                                            }
                                        }
                                    }
                                }
                            }
                            
                            if include {
                                filtered_asteroids.push(asteroid.clone());
                            }
                        }
                    }
                }
                
                Ok(Some(json!({
                    "element_count": filtered_asteroids.len(),
                    "filters_applied": {
                        "hazardous": sub_matches.get_flag("hazardous"),
                        "size": sub_matches.get_one::<String>("size"),
                        "distance_range": {
                            "min": sub_matches.get_one::<String>("min-distance"),
                            "max": sub_matches.get_one::<String>("max-distance")
                        }
                    },
                    "asteroids": filtered_asteroids
                })))
            } else {
                Ok(Some(data))
            }
        }
        Some(("batch-lookup", sub_matches)) => {
            let ids: Vec<&str> = sub_matches.get_many::<String>("ids")
                .unwrap()
                .map(|s| s.as_str())
                .collect();
            
            let mut asteroids = Vec::new();
            
            for id in ids {
                let params = HashMap::new();
                if let Ok(data) = client.get(&format!("/api/neo/{id}"), params).await {
                    asteroids.push(json!({
                        "id": id,
                        "data": data
                    }));
                } else {
                    asteroids.push(json!({
                        "id": id,
                        "error": "Failed to retrieve asteroid data"
                    }));
                }
            }
            
            Ok(Some(json!({
                "count": asteroids.len(),
                "asteroids": asteroids
            })))
        }
        Some(("weekly-summary", sub_matches)) => {
            let week_offset = sub_matches.get_one::<String>("week-offset")
                .unwrap()
                .parse::<i64>()
                .unwrap_or(0);
            
            // Calculate week start and end
            let today = Local::now().date_naive();
            let days_since_monday = today.weekday().num_days_from_monday() as i64;
            let week_start = today - Duration::days(days_since_monday) + Duration::weeks(week_offset);
            let week_end = week_start + Duration::days(6);
            
            let mut params = HashMap::new();
            params.insert("start_date".to_string(), week_start.format("%Y-%m-%d").to_string());
            params.insert("end_date".to_string(), week_end.format("%Y-%m-%d").to_string());
            
            let data = client.get("/api/neo/feed", params).await?;
            
            // Summarize the data
            if let Some(neo_objects) = data.get("near_earth_objects").and_then(|n| n.as_object()) {
                let mut total_asteroids = 0;
                let mut hazardous_count = 0;
                let mut closest_approach = f64::MAX;
                let mut closest_asteroid = String::new();
                
                for (_date, asteroids) in neo_objects {
                    if let Some(asteroid_list) = asteroids.as_array() {
                        total_asteroids += asteroid_list.len();
                        
                        for asteroid in asteroid_list {
                            if let Some(hazardous) = asteroid.get("is_potentially_hazardous_asteroid").and_then(|h| h.as_bool()) {
                                if hazardous {
                                    hazardous_count += 1;
                                }
                            }
                            
                            if let Some(approaches) = asteroid.get("close_approach_data").and_then(|c| c.as_array()) {
                                if let Some(first_approach) = approaches.first() {
                                    if let Some(miss_distance) = first_approach.get("miss_distance")
                                        .and_then(|m| m.get("astronomical"))
                                        .and_then(|a| a.as_str())
                                        .and_then(|s| s.parse::<f64>().ok())
                                    {
                                        if miss_distance < closest_approach {
                                            closest_approach = miss_distance;
                                            closest_asteroid = asteroid.get("name")
                                                .and_then(|n| n.as_str())
                                                .unwrap_or("Unknown")
                                                .to_string();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                Ok(Some(json!({
                    "week_of": week_start.format("%Y-%m-%d").to_string(),
                    "week_ending": week_end.format("%Y-%m-%d").to_string(),
                    "summary": {
                        "total_asteroids": total_asteroids,
                        "hazardous_asteroids": hazardous_count,
                        "closest_approach": {
                            "distance_au": closest_approach,
                            "asteroid": closest_asteroid
                        }
                    },
                    "element_count": data.get("element_count")
                })))
            } else {
                Ok(Some(data))
            }
        }
        _ => {
            eprintln!("Please specify a subcommand: feed, lookup, browse, feed-extended, batch-lookup, or weekly-summary");
            Ok(None)
        }
    }
}