use std::collections::HashMap;
use serde_json::{json, Value};

// Helper functions
fn is_valid_date_format(date: &str) -> bool {
    use chrono::NaiveDate;
    NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}

fn is_valid_coordinate(lat: f64, lon: f64) -> bool {
    (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon)
}

fn is_valid_camera_for_rover(rover: &str, camera: &str) -> bool {
    match rover {
        "curiosity" => matches!(camera, "FHAZ" | "RHAZ" | "MAST" | "CHEMCAM" | "MAHLI" | "MARDI" | "NAVCAM"),
        "opportunity" | "spirit" => matches!(camera, "FHAZ" | "RHAZ" | "NAVCAM" | "PANCAM" | "MINITES"),
        _ => false,
    }
}

fn parse_rate_limit_headers(headers: &HashMap<String, String>) -> (Option<u32>, Option<u32>) {
    let limit = headers.get("X-RateLimit-Limit")
        .and_then(|v| v.parse::<u32>().ok());
    let remaining = headers.get("X-RateLimit-Remaining")
        .and_then(|v| v.parse::<u32>().ok());
    (limit, remaining)
}

fn generate_cache_key(endpoint: &str, params: &[(&str, &str)]) -> String {
    let mut sorted_params = params.to_vec();
    sorted_params.sort_by_key(|&(k, _)| k);
    
    let param_string = sorted_params
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");
    
    format!("{endpoint}:{param_string}")
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    // Mock NASA API responses for testing
    fn mock_apod_response() -> Value {
        json!({
            "copyright": "John Doe",
            "date": "2024-01-01",
            "explanation": "A beautiful nebula in the night sky...",
            "hdurl": "https://apod.nasa.gov/apod/image/2401/nebula_hd.jpg",
            "media_type": "image",
            "service_version": "v1",
            "title": "The Cosmic Nebula",
            "url": "https://apod.nasa.gov/apod/image/2401/nebula.jpg"
        })
    }
    
    fn mock_neo_feed_response() -> Value {
        json!({
            "links": {
                "self": "https://api.nasa.gov/neo/rest/v1/feed"
            },
            "element_count": 2,
            "near_earth_objects": {
                "2024-01-01": [
                    {
                        "id": "54321",
                        "name": "(2023 AB1)",
                        "absolute_magnitude_h": 22.1,
                        "is_potentially_hazardous_asteroid": false,
                        "close_approach_data": [{
                            "close_approach_date": "2024-01-01",
                            "relative_velocity": {
                                "kilometers_per_second": "15.123"
                            },
                            "miss_distance": {
                                "astronomical": "0.123"
                            }
                        }]
                    }
                ]
            }
        })
    }
    
    #[test]
    fn test_response_parsing_and_validation() {
        // Test APOD response parsing
        let apod_data = mock_apod_response();
        assert_eq!(apod_data["date"], "2024-01-01");
        assert_eq!(apod_data["media_type"], "image");
        assert!(apod_data["url"].as_str().unwrap().starts_with("https://"));
        
        // Test NEO response parsing
        let neo_data = mock_neo_feed_response();
        assert_eq!(neo_data["element_count"], 2);
        assert!(neo_data["near_earth_objects"]["2024-01-01"].is_array());
    }
    
    #[test]
    fn test_parameter_validation() {
        // Test date format validation
        let valid_dates = vec!["2024-01-01", "2023-12-31", "2022-06-15"];
        let invalid_dates = vec!["01-01-2024", "2024/01/01", "January 1, 2024", "not-a-date"];
        
        for date in valid_dates {
            assert!(is_valid_date_format(date), "Date {date} should be valid");
        }
        
        for date in invalid_dates {
            assert!(!is_valid_date_format(date), "Date {date} should be invalid");
        }
    }
    
    #[test]
    fn test_coordinate_validation() {
        // Valid coordinates
        assert!(is_valid_coordinate(29.78, -95.33));
        assert!(is_valid_coordinate(0.0, 0.0));
        assert!(is_valid_coordinate(-90.0, 180.0));
        
        // Invalid coordinates
        assert!(!is_valid_coordinate(91.0, 0.0));  // lat > 90
        assert!(!is_valid_coordinate(0.0, 181.0)); // lon > 180
        assert!(!is_valid_coordinate(-91.0, 0.0)); // lat < -90
        assert!(!is_valid_coordinate(0.0, -181.0)); // lon < -180
    }
    
    #[test]
    fn test_mars_rover_cameras() {
        let curiosity_cameras = vec!["FHAZ", "RHAZ", "MAST", "CHEMCAM", "MAHLI", "MARDI", "NAVCAM"];
        let opportunity_cameras = vec!["FHAZ", "RHAZ", "NAVCAM", "PANCAM", "MINITES"];
        
        for camera in &curiosity_cameras {
            assert!(is_valid_camera_for_rover("curiosity", camera));
        }
        
        for camera in &opportunity_cameras {
            assert!(is_valid_camera_for_rover("opportunity", camera));
        }
        
        // Test invalid combinations
        assert!(!is_valid_camera_for_rover("curiosity", "PANCAM")); // Opportunity-only camera
        assert!(!is_valid_camera_for_rover("opportunity", "MAST")); // Curiosity-only camera
    }
    
    #[test]
    fn test_rate_limit_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-RateLimit-Limit".to_string(), "1000".to_string());
        headers.insert("X-RateLimit-Remaining".to_string(), "950".to_string());
        
        let (limit, remaining) = parse_rate_limit_headers(&headers);
        assert_eq!(limit, Some(1000));
        assert_eq!(remaining, Some(950));
    }
    
    #[test]
    fn test_cache_key_consistency() {
        // Test that cache keys are consistent regardless of parameter order
        let params1 = vec![
            ("date", "2024-01-01"),
            ("hd", "true"),
            ("count", "5"),
        ];
        
        let params2 = vec![
            ("hd", "true"),
            ("count", "5"),
            ("date", "2024-01-01"),
        ];
        
        let key1 = generate_cache_key("/api/apod", &params1);
        let key2 = generate_cache_key("/api/apod", &params2);
        
        assert_eq!(key1, key2, "Cache keys should be identical regardless of parameter order");
    }
    
    #[test]
    fn test_error_response_format() {
        let error_types = vec![
            ("BadRequest", 400),
            ("Unauthorized", 401),
            ("NotFound", 404),
            ("RateLimitExceeded", 429),
            ("InternalServerError", 500),
        ];
        
        for (error_type, _expected_code) in error_types {
            let error_response = json!({
                "error": error_type,
                "message": "Test error message"
            });
            
            assert_eq!(
                error_response["error"], 
                error_type,
                "Error type should match"
            );
        }
    }
}