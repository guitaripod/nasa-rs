#[cfg(test)]
#[cfg(feature = "cli")]
mod cli_tests {
    use crate::cli::{
        output::{OutputFormat, Formatter},
        cache::LocalCache,
        config::{Config, ConfigManager},
    };
    use serde_json::json;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_output_formats() {
        let data = json!({
            "title": "Test Title",
            "date": "2024-01-01",
            "explanation": "Test explanation"
        });

        // Test JSON format
        let json_formatter = Formatter::new(OutputFormat::Json);
        let json_output = json_formatter.format(&data).unwrap();
        assert!(json_output.contains("\"title\": \"Test Title\""));

        // Test Pretty format
        let pretty_formatter = Formatter::new(OutputFormat::Pretty);
        let pretty_output = pretty_formatter.format(&data).unwrap();
        assert!(pretty_output.contains("Test Title"));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config {
            api_endpoint: "https://test.workers.dev".to_string(),
            api_key: Some("test_key".to_string()),
            output_format: OutputFormat::Json,
            use_cache: true,
            cache_dir: PathBuf::from("/tmp/cache"),
            cache_ttl_minutes: 60,
        };

        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("api_endpoint = \"https://test.workers.dev\""));
        assert!(toml_str.contains("output_format = \"json\""));
        assert!(toml_str.contains("use_cache = true"));

        // Test deserialization
        let config2: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.api_endpoint, config2.api_endpoint);
        assert_eq!(config.output_format, config2.output_format);
    }

    #[tokio::test]
    async fn test_local_cache() {
        let temp_dir = TempDir::new().unwrap();
        let cache = LocalCache::new(temp_dir.path().to_path_buf(), 60);

        let test_data = json!({
            "test": "data",
            "number": 42
        });

        // Test cache miss
        let result = cache.get("test_key").await.unwrap();
        assert!(result.is_none());

        // Test cache set and get
        cache.set("test_key", &test_data).await.unwrap();
        let cached = cache.get("test_key").await.unwrap();
        assert!(cached.is_some());
        assert_eq!(cached.unwrap(), test_data);
    }

    #[test]
    fn test_cache_key_generation() {
        use crate::cli::cache::LocalCache;
        
        let key1 = LocalCache::generate_cache_key("/api/apod", &[("date", "2024-01-01")]);
        let key2 = LocalCache::generate_cache_key("/api/apod", &[("date", "2024-01-01")]);
        assert_eq!(key1, key2);

        // Different params should generate different keys
        let key3 = LocalCache::generate_cache_key("/api/apod", &[("date", "2024-01-02")]);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_command_argument_parsing() {
        use clap::{Command, Arg};
        
        let app = Command::new("test")
            .arg(Arg::new("date").long("date").help("Test date"));
        
        let matches = app.try_get_matches_from(vec!["test", "--date", "2024-01-01"]).unwrap();
        assert_eq!(matches.get_one::<String>("date").unwrap(), "2024-01-01");
    }

    #[test]
    fn test_date_validation() {
        use chrono::NaiveDate;
        
        // Valid date
        let valid = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d");
        assert!(valid.is_ok());
        
        // Invalid date
        let invalid = NaiveDate::parse_from_str("2024-13-01", "%Y-%m-%d");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_csv_formatter_with_table_data() {
        let data = json!([
            {"name": "Object 1", "distance": 0.05, "velocity": 15.2},
            {"name": "Object 2", "distance": 0.12, "velocity": 22.8},
            {"name": "Object 3", "distance": 0.08, "velocity": 18.5}
        ]);

        let formatter = Formatter::new(OutputFormat::Csv);
        let output = formatter.format(&data).unwrap();
        
        // Check CSV headers
        assert!(output.contains("name,distance,velocity"));
        // Check data rows
        assert!(output.contains("Object 1,0.05,15.2"));
        assert!(output.contains("Object 2,0.12,22.8"));
    }
}