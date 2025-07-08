use clap::ArgMatches;
use serde_json::{Value, json};
use colored::Colorize;
use crate::cli::commands::CommandContext;
use crate::cli::output::OutputFormat;

pub async fn execute(
    matches: &ArgMatches,
    context: &CommandContext,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("show", _)) => {
            let config = &context.config;
            let config_json = json!({
                "api_endpoint": config.api_endpoint,
                "api_key": config.api_key.as_ref().map(|_| "<hidden>"),
                "output_format": format!("{:?}", config.output_format),
                "use_cache": config.use_cache,
                "cache_dir": config.cache_dir.display().to_string(),
                "cache_ttl_minutes": config.cache_ttl_minutes,
            });
            
            Ok(Some(config_json))
        }
        Some(("set", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").unwrap();
            let value = sub_matches.get_one::<String>("value").unwrap();
            
            let mut config = context.config.clone();
            
            match key.as_str() {
                "api_endpoint" => config.api_endpoint = value.clone(),
                "api_key" => config.api_key = Some(value.clone()),
                "output_format" => {
                    config.output_format = match value.as_str() {
                        "json" => OutputFormat::Json,
                        "table" => OutputFormat::Table,
                        "pretty" => OutputFormat::Pretty,
                        "csv" => OutputFormat::Csv,
                        _ => return Err(format!("Invalid output format: {value}").into()),
                    };
                }
                "use_cache" => config.use_cache = value.parse::<bool>()?,
                "cache_ttl_minutes" => config.cache_ttl_minutes = value.parse::<u64>()?,
                _ => return Err(format!("Unknown configuration key: {key}").into()),
            }
            
            context.config_manager.save(&config).await?;
            println!("{} {} = {}", "Set".green().bold(), key, value);
            
            Ok(None)
        }
        Some(("init", _)) => {
            context.config_manager.init().await?;
            println!("{} Configuration initialized", "✓".green().bold());
            
            Ok(None)
        }
        _ => {
            eprintln!("Please specify a subcommand: show, set, or init");
            Ok(None)
        }
    }
}