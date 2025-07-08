mod apod;
mod asteroids;
mod cache;
mod config;
mod donki;
mod earth;
mod epic;
mod exoplanets;
mod mars;
mod media;
mod ssd;
mod tech;

use clap::ArgMatches;
use crate::cli::{
    config::{Config, ConfigManager},
    api::ApiClient,
    output::create_formatter,
};

pub struct CommandContext {
    pub config: Config,
    pub config_manager: ConfigManager,
}

pub async fn execute_command(
    command: &str,
    matches: &ArgMatches,
    context: &CommandContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // For interactive mode, handle it separately
    if command == "interactive" {
        crate::cli::interactive::run_interactive_mode(context).await?;
        return Ok(());
    }
    
    // Create API client
    let client = ApiClient::new(
        context.config.api_endpoint.clone(),
        context.config.api_key.clone(),
        context.config.cache_dir.clone(),
        context.config.use_cache,
        context.config.cache_ttl_minutes,
    );
    
    // Execute the appropriate command
    let result = match command {
        "apod" => apod::execute(matches, &client).await,
        "asteroids" => asteroids::execute(matches, &client).await,
        "donki" => donki::execute(matches, &client).await,
        "mars" => mars::execute(matches, &client).await,
        "earth" => earth::execute(matches, &client).await,
        "epic" => epic::execute(matches, &client).await,
        "tech" => tech::execute(matches, &client).await,
        "media" => media::execute(matches, &client).await,
        "exoplanets" => exoplanets::execute(matches, &client).await,
        "ssd" => ssd::execute(matches, &client).await,
        "config" => config::execute(matches, context).await,
        "cache" => cache::execute(matches, &client).await,
        _ => Err(format!("Unknown command: {command}").into()),
    };
    
    match result {
        Ok(Some(data)) => {
            // Format and display the output
            let formatter = create_formatter(context.config.output_format);
            let output = formatter.format(&data)?;
            println!("{output}");
        }
        Ok(None) => {
            // Command handled output internally
        }
        Err(e) => {
            return Err(e);
        }
    }
    
    Ok(())
}