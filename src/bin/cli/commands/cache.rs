use clap::ArgMatches;
use serde_json::Value;
use colored::Colorize;
use crate::cli::api::ApiClient;

pub async fn execute(
    matches: &ArgMatches,
    client: &ApiClient,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("clear", _)) => {
            client.clear_cache().await?;
            println!("{} Cache cleared", "✓".green().bold());
            Ok(None)
        }
        Some(("stats", _)) => {
            let (count, size) = client.get_cache_stats().await?;
            
            println!("{}", "Cache Statistics".bold().cyan());
            println!("{}", "─".repeat(20));
            println!("{}: {}", "Files".green(), count);
            println!("{}: {} bytes ({:.2} MB)", 
                "Total Size".green(), 
                size,
                size as f64 / 1_048_576.0
            );
            
            Ok(None)
        }
        _ => {
            eprintln!("Please specify a subcommand: clear or stats");
            Ok(None)
        }
    }
}