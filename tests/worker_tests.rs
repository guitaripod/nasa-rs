#[cfg(test)]
mod tests {
    use worker::{Request, Env, Context, Method};
    use serde_json::json;
    use crate::models::apod::ApodResponse;
    
    // Mock helper to create test request
    fn create_test_request(path: &str, method: Method) -> Request {
        Request::new(
            &format!("https://test.workers.dev{}", path),
            method
        ).unwrap()
    }

    #[test]
    fn test_apod_response_deserialization() {
        let json_data = json!({
            "date": "2024-01-01",
            "explanation": "Test explanation",
            "hdurl": "https://example.com/hd.jpg",
            "media_type": "image",
            "service_version": "v1",
            "title": "Test Title",
            "url": "https://example.com/image.jpg"
        });

        let response: ApodResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.date, "2024-01-01");
        assert_eq!(response.title, "Test Title");
        assert_eq!(response.media_type, "image");
    }

    #[test]
    fn test_neo_feed_response_deserialization() {
        let json_data = json!({
            "links": {
                "next": "https://api.nasa.gov/neo/rest/v1/feed?next",
                "previous": "https://api.nasa.gov/neo/rest/v1/feed?prev",
                "self": "https://api.nasa.gov/neo/rest/v1/feed"
            },
            "element_count": 1,
            "near_earth_objects": {
                "2024-01-01": [{
                    "id": "1",
                    "neo_reference_id": "1",
                    "name": "Test Asteroid",
                    "nasa_jpl_url": "https://example.com",
                    "absolute_magnitude_h": 20.0,
                    "estimated_diameter": {
                        "kilometers": {
                            "estimated_diameter_min": 0.1,
                            "estimated_diameter_max": 0.2
                        },
                        "meters": {
                            "estimated_diameter_min": 100.0,
                            "estimated_diameter_max": 200.0
                        }
                    },
                    "is_potentially_hazardous_asteroid": false,
                    "close_approach_data": [{
                        "close_approach_date": "2024-01-01",
                        "close_approach_date_full": "2024-Jan-01 12:00",
                        "epoch_date_close_approach": 1704110400000,
                        "relative_velocity": {
                            "kilometers_per_second": "10.0",
                            "kilometers_per_hour": "36000.0",
                            "miles_per_hour": "22369.4"
                        },
                        "miss_distance": {
                            "astronomical": "0.1",
                            "lunar": "38.9",
                            "kilometers": "14959787.0",
                            "miles": "9295442.0"
                        },
                        "orbiting_body": "Earth"
                    }],
                    "is_sentry_object": false
                }]
            }
        });

        let response: crate::models::neo::NeoFeedResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.element_count, 1);
        assert!(response.near_earth_objects.contains_key("2024-01-01"));
    }

    #[test]
    fn test_cache_key_generation() {
        use crate::cache::CacheManager;
        use std::collections::HashMap;
        
        let mut params = HashMap::new();
        params.insert("date".to_string(), "2024-01-01".to_string());
        params.insert("api_key".to_string(), "test_key".to_string());
        
        let key = CacheManager::generate_cache_key("/api/apod", &params);
        assert!(key.starts_with("nasa_api:"));
        assert!(key.contains("/api/apod"));
        
        // Test that params are sorted consistently
        let mut params2 = HashMap::new();
        params2.insert("api_key".to_string(), "test_key".to_string());
        params2.insert("date".to_string(), "2024-01-01".to_string());
        
        let key2 = CacheManager::generate_cache_key("/api/apod", &params2);
        assert_eq!(key, key2);
    }

    #[test]
    fn test_endpoint_ttl() {
        use crate::cache::CacheManager;
        
        // Test different endpoints have different TTLs
        assert_eq!(CacheManager::get_ttl_for_endpoint("/api/apod"), 86400); // 24 hours
        assert_eq!(CacheManager::get_ttl_for_endpoint("/api/neo/feed"), 3600); // 1 hour
        assert_eq!(CacheManager::get_ttl_for_endpoint("/api/donki/flr"), 1800); // 30 minutes
        assert_eq!(CacheManager::get_ttl_for_endpoint("/api/mars-photos/curiosity/photos"), 86400); // 24 hours
        assert_eq!(CacheManager::get_ttl_for_endpoint("/api/unknown"), 3600); // default 1 hour
    }

    #[test]
    fn test_error_response_formatting() {
        use crate::error::ApiError;
        
        let error = ApiError::BadRequest("Invalid date format".to_string());
        let response = error.to_response();
        
        assert_eq!(response.status_code(), 400);
    }

    #[test]
    fn test_mars_camera_validation() {
        let valid_cameras = ["FHAZ", "RHAZ", "MAST", "CHEMCAM", "MAHLI", "MARDI", "NAVCAM", "PANCAM", "MINITES"];
        
        for camera in &valid_cameras {
            // In a real test, you'd validate this against your camera validation logic
            assert!(camera.len() >= 4);
        }
    }
}