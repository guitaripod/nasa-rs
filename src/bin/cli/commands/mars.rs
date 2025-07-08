use clap::ArgMatches;
use serde_json::Value;
use std::collections::HashMap;
use crate::cli::api::ApiClient;

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
            
            let data = client.get(&format!("/api/mars-photos/{}/photos", rover), params).await?;
            Ok(Some(data))
        }
        Some(("latest", sub_matches)) => {
            let rover = sub_matches.get_one::<String>("rover").unwrap();
            let params = HashMap::new();
            
            let data = client.get(&format!("/api/mars-photos/{}/latest", rover), params).await?;
            Ok(Some(data))
        }
        Some(("manifest", sub_matches)) => {
            let rover = sub_matches.get_one::<String>("rover").unwrap();
            let params = HashMap::new();
            
            let data = client.get(&format!("/api/mars-photos/manifests/{}", rover), params).await?;
            Ok(Some(data))
        }
        _ => {
            eprintln!("Please specify a subcommand: photos, latest, or manifest");
            Ok(None)
        }
    }
}