#![cfg(feature = "cli")]

use clap::{Command, Arg, ArgAction};
use colored::Colorize;
use std::process;

mod cli;
use self::cli::{
    config::ConfigManager,
    commands::{execute_command, CommandContext},
    output::OutputFormat,
};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = build_cli();
    let matches = app.get_matches();
    
    // Load configuration
    let config_manager = ConfigManager::new()?;
    let mut config = config_manager.load().await?;
    
    // Override with command line options
    if let Some(output) = matches.get_one::<String>("output") {
        config.output_format = match output.as_str() {
            "json" => OutputFormat::Json,
            "table" => OutputFormat::Table,
            "pretty" => OutputFormat::Pretty,
            "csv" => OutputFormat::Csv,
            _ => config.output_format,
        };
    }
    
    if let Some(endpoint) = matches.get_one::<String>("endpoint") {
        config.api_endpoint = endpoint.clone();
    }
    
    if matches.get_flag("no-cache") {
        config.use_cache = false;
    }
    
    // Create command context
    let context = CommandContext {
        config: config.clone(),
        config_manager,
    };
    
    // Handle subcommands
    match matches.subcommand() {
        Some((name, sub_matches)) => {
            execute_command(name, sub_matches, &context).await?;
        }
        None => {
            eprintln!("No command specified. Use --help for usage information.");
            process::exit(1);
        }
    }
    
    Ok(())
}

fn build_cli() -> Command {
    Command::new("nasa")
        .version("1.0.0")
        .author("NASA API CLI")
        .about("Command-line interface for NASA APIs")
        .arg_required_else_help(true)
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Output format")
                .value_parser(["json", "table", "pretty", "csv"])
                .global(true),
        )
        .arg(
            Arg::new("endpoint")
                .long("endpoint")
                .help("API endpoint URL")
                .global(true),
        )
        .arg(
            Arg::new("no-cache")
                .long("no-cache")
                .help("Disable response caching")
                .action(ArgAction::SetTrue)
                .global(true),
        )
        // APOD commands
        .subcommand(
            Command::new("apod")
                .about("Astronomy Picture of the Day")
                .subcommand(
                    Command::new("today")
                        .about("Get today's APOD")
                )
                .subcommand(
                    Command::new("date")
                        .about("Get APOD for a specific date")
                        .arg(Arg::new("date").required(true).help("Date (YYYY-MM-DD)"))
                )
                .subcommand(
                    Command::new("random")
                        .about("Get random APOD images")
                        .arg(Arg::new("count").default_value("1").help("Number of images"))
                )
        )
        // Asteroids commands
        .subcommand(
            Command::new("asteroids")
                .about("Near Earth Objects (asteroids)")
                .subcommand(
                    Command::new("feed")
                        .about("Get asteroid feed")
                        .arg(Arg::new("start-date").help("Start date (YYYY-MM-DD)"))
                        .arg(Arg::new("end-date").help("End date (YYYY-MM-DD)"))
                )
                .subcommand(
                    Command::new("lookup")
                        .about("Lookup specific asteroid")
                        .arg(Arg::new("id").required(true).help("Asteroid ID"))
                )
                .subcommand(
                    Command::new("browse")
                        .about("Browse asteroid database")
                        .arg(Arg::new("page").default_value("1").help("Page number"))
                        .arg(Arg::new("size").default_value("20").help("Page size"))
                )
        )
        // DONKI (Space Weather) commands
        .subcommand(
            Command::new("donki")
                .about("Space Weather Database")
                .subcommand(
                    Command::new("cme")
                        .about("Coronal Mass Ejections")
                        .arg(Arg::new("start-date").help("Start date"))
                        .arg(Arg::new("end-date").help("End date"))
                )
                .subcommand(
                    Command::new("flare")
                        .about("Solar Flares")
                        .arg(Arg::new("start-date").help("Start date"))
                        .arg(Arg::new("end-date").help("End date"))
                )
                .subcommand(
                    Command::new("storm")
                        .about("Geomagnetic Storms")
                        .arg(Arg::new("start-date").help("Start date"))
                        .arg(Arg::new("end-date").help("End date"))
                )
                .subcommand(
                    Command::new("notifications")
                        .about("Space weather notifications")
                        .arg(Arg::new("type").help("Notification type"))
                )
        )
        // Mars commands
        .subcommand(
            Command::new("mars")
                .about("Mars rover photos")
                .subcommand(
                    Command::new("photos")
                        .about("Get rover photos")
                        .arg(Arg::new("rover").required(true).help("Rover name (curiosity, opportunity, spirit)"))
                        .arg(Arg::new("sol").help("Martian sol"))
                        .arg(Arg::new("earth-date").help("Earth date (YYYY-MM-DD)"))
                        .arg(Arg::new("camera").help("Camera abbreviation"))
                        .arg(Arg::new("page").default_value("1").help("Page number"))
                )
                .subcommand(
                    Command::new("latest")
                        .about("Get latest photos")
                        .arg(Arg::new("rover").required(true).help("Rover name"))
                )
                .subcommand(
                    Command::new("manifest")
                        .about("Get mission manifest")
                        .arg(Arg::new("rover").required(true).help("Rover name"))
                )
        )
        // Earth imagery commands  
        .subcommand(
            Command::new("earth")
                .about("Earth satellite imagery")
                .subcommand(
                    Command::new("image")
                        .about("Get satellite image")
                        .arg(Arg::new("lat").required(true).help("Latitude"))
                        .arg(Arg::new("lon").required(true).help("Longitude"))
                        .arg(Arg::new("date").help("Date (YYYY-MM-DD)"))
                        .arg(Arg::new("dim").help("Image dimension in degrees"))
                )
                .subcommand(
                    Command::new("assets")
                        .about("Get available asset dates")
                        .arg(Arg::new("lat").required(true).help("Latitude"))
                        .arg(Arg::new("lon").required(true).help("Longitude"))
                        .arg(Arg::new("date").required(true).help("Date (YYYY-MM-DD)"))
                )
        )
        // EPIC commands
        .subcommand(
            Command::new("epic")
                .about("Earth Polychromatic Imaging Camera")
                .subcommand(
                    Command::new("natural")
                        .about("Natural color images")
                        .arg(Arg::new("date").help("Date (YYYY-MM-DD) or 'all' for available dates"))
                )
                .subcommand(
                    Command::new("enhanced")
                        .about("Enhanced color images")
                        .arg(Arg::new("date").help("Date (YYYY-MM-DD) or 'all' for available dates"))
                )
        )
        // Tech Transfer commands
        .subcommand(
            Command::new("tech")
                .about("NASA technology transfer")
                .subcommand(
                    Command::new("patents")
                        .about("Search patents")
                        .arg(Arg::new("query").help("Search query"))
                )
                .subcommand(
                    Command::new("software")
                        .about("Search software")
                        .arg(Arg::new("query").help("Search query"))
                )
                .subcommand(
                    Command::new("spinoffs")
                        .about("Search spinoff technologies")
                        .arg(Arg::new("query").help("Search query"))
                )
        )
        // Media commands
        .subcommand(
            Command::new("media")
                .about("NASA Image and Video Library")
                .subcommand(
                    Command::new("search")
                        .about("Search media")
                        .arg(Arg::new("query").help("Search query"))
                        .arg(Arg::new("media-type").help("Media type (image, video, audio)"))
                        .arg(Arg::new("center").help("NASA center"))
                        .arg(Arg::new("year-start").help("Start year"))
                        .arg(Arg::new("year-end").help("End year"))
                        .arg(Arg::new("page").default_value("1").help("Page number"))
                )
                .subcommand(
                    Command::new("asset")
                        .about("Get asset details")
                        .arg(Arg::new("nasa-id").required(true).help("NASA ID"))
                )
        )
        // Exoplanets commands
        .subcommand(
            Command::new("exoplanets")
                .about("Exoplanet data")
                .subcommand(
                    Command::new("search")
                        .about("Search exoplanets")
                        .arg(Arg::new("query").required(true).help("ADQL query"))
                        .arg(Arg::new("format").default_value("json").help("Output format"))
                )
                .subcommand(
                    Command::new("kepler")
                        .about("Search Kepler discoveries")
                        .arg(Arg::new("name").help("Planet name pattern"))
                )
        )
        // SSD commands
        .subcommand(
            Command::new("ssd")
                .about("Solar System Dynamics")
                .subcommand(
                    Command::new("close-approach")
                        .about("Close approach data")
                        .arg(Arg::new("date-min").help("Minimum date"))
                        .arg(Arg::new("date-max").help("Maximum date"))
                        .arg(Arg::new("dist-max").help("Maximum distance (AU)"))
                        .arg(Arg::new("pha").help("Only potentially hazardous").action(ArgAction::SetTrue))
                )
                .subcommand(
                    Command::new("fireballs")
                        .about("Fireball atmospheric impact data")
                        .arg(Arg::new("date-min").help("Minimum date"))
                        .arg(Arg::new("date-max").help("Maximum date"))
                        .arg(Arg::new("req-loc").help("Require location data").action(ArgAction::SetTrue))
                )
                .subcommand(
                    Command::new("sentry")
                        .about("Impact risk assessment")
                        .arg(Arg::new("object").help("Object designation"))
                )
        )
        // Config commands
        .subcommand(
            Command::new("config")
                .about("Manage configuration")
                .subcommand(
                    Command::new("show")
                        .about("Show current configuration")
                )
                .subcommand(
                    Command::new("set")
                        .about("Set configuration value")
                        .arg(Arg::new("key").required(true).help("Configuration key"))
                        .arg(Arg::new("value").required(true).help("Configuration value"))
                )
                .subcommand(
                    Command::new("init")
                        .about("Initialize configuration")
                )
        )
        // Cache commands
        .subcommand(
            Command::new("cache")
                .about("Manage response cache")
                .subcommand(
                    Command::new("clear")
                        .about("Clear cache")
                )
                .subcommand(
                    Command::new("stats")
                        .about("Show cache statistics")
                )
        )
        // Interactive mode
        .subcommand(
            Command::new("interactive")
                .about("Launch interactive mode")
                .alias("i")
        )
}