#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

#[cfg(debug_assertions)]
extern crate better_panic;

use utils::app_config::AppConfig;
use utils::error::Result;
use utils::logger::install_logger;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// does running things
    Run {
        /// lists run values?
        #[arg(short, long)]
        list: bool,
    },
    Config {

    },
    Error {

    }
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub database: Database,
}

use cliargs::{Cli, Commands::*};

/// Match commands
pub fn cli_match() -> Result<()> {
    // Get matches
    let Cli{config, command} = Cli::parse();

    // Merge clap config file if the value is set
    AppConfig::merge_config(config)?;

    // Matches Commands or display help
    return match command {
        Test { list } => Ok(println!("Tests!")),
        Run { list } => Ok(println!("Runs!")),
        Config {  } => commands::config(),
        Error {  } => commands::simulate_error(),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

fn main() -> Result<()> {

    let default_config = "resources/default_config.toml";
    let env_prefix = "APP";
    let config = utils::common_inits::common_inits(default_config, env_prefix)?;
    let config = &mut config;
    cli::cli_match()
}
