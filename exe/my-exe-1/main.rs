#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

#[cfg(debug_assertions)]
extern crate better_panic;

use serde::Deserialize;
use utils::cli_config::CliConfig;
use std::{collections::HashMap, error::Error, path::PathBuf};
use clap::Parser;
use midi_mapper::{chordifier::{Chordifier, Note}, chords::{make_mapping, ChordType}, midi_device::create_virtual_midi_device};

#[derive(Debug, Deserialize, Clone)]
struct Profile { roots : Vec<(Note, Note)>, chord_types : Vec<(Note, ChordType)> }

#[derive(Parser, Debug, Deserialize)]
#[command(author, version, about, long_about = "A virtual device that maps root note + chord type to a chord")]
pub struct Cli {
  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  pub config: Option<PathBuf>,

  /// Sets a custom config file
  #[arg(short, long, value_name = "NAME", default_value="default")]
  pub profile: String,

  /// Sets device name
  #[arg(short, long, value_name = "NAME", default_value="midi-mapper")]
  pub name : String
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
  pub args: Cli,
  pub profiles: HashMap<String, Profile>,
}

fn run( device_name: String, profile : Profile ) -> Result<(), Box<dyn Error>> {
  let chords_map = make_mapping(profile.roots, profile.chord_types);
  let chordifier = Chordifier::new(chords_map);
  create_virtual_midi_device(
    device_name.as_str(),
    chordifier
  )
}

/// Match commands
pub fn cli_match(config: CliConfig) -> Result<(), Box<dyn Error>> {
  // let (cliconfig, command) =
  // {
  let mut config1 = config;
  let cliconfig = &mut config1;

  // Get matches
  let Cli { config, name: _name, profile: _profile } = Cli::parse();

  // Merge clap config file if the value is set
  cliconfig.merge_config(config)?;

  let appconfig: AppConfig = config1.fetch()?;
  let profile_name = appconfig.args.profile;
  let profile = appconfig.profiles.get(&profile_name);
  match profile {
    Some(profile) => run(appconfig.args.name, profile.clone()),
    None => Err(format!("Profile {} not found", profile_name).into())
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let config_contents = include_str!("resources/default_config.toml");
  let env_prefix = "APP";
  let config = utils::common_inits::common_inits(config_contents, env_prefix)
    // eh...
    .map_err(|e| format!("{:?}", e))?;
  // let config = &mut config;
  cli_match(config)
}

#[test]
fn verify_cli() {
  use clap::CommandFactory;
  Cli::command().debug_assert()
}
