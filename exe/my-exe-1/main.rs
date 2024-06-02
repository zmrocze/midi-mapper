#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

#[cfg(debug_assertions)]
extern crate better_panic;

use utils::cli_config::CliConfig;
use utils::error::Result;
use serde::Deserialize;

use std::{fs::File, path::PathBuf};

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

/// Match commands
pub fn cli_match(config: CliConfig) -> Result<()> {
    // let (cliconfig, command) = 
    // { 
        let mut config1 = config; 
    let cliconfig = &mut config1;

    // Get matches
    let Cli{config, command} = Cli::parse();
    
    // Merge clap config file if the value is set
    cliconfig.merge_config(config)?;
    // (config1, command)
    // };
    use Commands::*;
    // Matches Commands or display help
    return match command {
        Test { list: _list } => Ok(println!("Tests!")),
        Run { list: _list } => Ok(println!("Runs!")),
        Config {  } => {
            let appconfig: AppConfig = config1.fetch()?;
            Ok(println!("{:#?}", appconfig))
        },
        Error {  } => {
            tracing::info!("We are simulating an error");
            {
                File::open("thisfiledoesnotexist")?; Ok::<(), utils::error::Error>(())
            }?;
            Ok(())
        
        },
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

fn main() -> Result<()> {

    let config_contents = include_str!("resources/default_config.toml");
    let env_prefix = "APP";
    let config = utils::common_inits::common_inits(config_contents, env_prefix)?;
    // let config = &mut config;
    cli_match(config)
}
