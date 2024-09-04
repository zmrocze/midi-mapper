use clap::Parser;
use midi_mapper::{
  chordifier::{ChannelChord, Chordifier, ChordsMap, Note},
  midi_device::create_virtual_midi_device,
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs::read_to_string, path::PathBuf};

#[derive(Parser, Debug, Deserialize)]
#[command(
  author,
  version,
  about,
  long_about = "A virtual device that maps root note + chord type to a chord"
)]
pub struct Cli {
  /// Sets a custom config file
  #[arg(short, long, value_name = "FILE")]
  pub config: Option<PathBuf>,

  /// Sets a custom config file
  #[arg(short, long, value_name = "NAME")]
  pub profile: Option<String>,

  /// Sets device name
  #[arg(short, long, value_name = "NAME")]
  pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
  pub name: String,
  pub profile: Profile,
  pub profile_name: String,
}

#[derive(Debug, Deserialize, Clone)]
struct KeyVal<K, V> {
  key: K,
  val: V,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
  map : Vec<KeyVal<ChannelChord, ChannelChord>>
}

#[derive(Debug, Deserialize)]
pub struct Profiles {
  profiles: HashMap<String, Profile>,
}

impl Default for Profiles {
  fn default() -> Self {
    Profiles {
      profiles: HashMap::new(),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
  pub name: Option<String>,
  pub profile: Option<String>,

  #[serde(flatten)]
  pub profiles: Profiles,
}

impl ConfigFile {
  // merge configs where the second overwrites the first
  pub fn merge(self, other: ConfigFile) -> ConfigFile {
    let mut profiles = self.profiles.profiles;
    profiles.extend(other.profiles.profiles);
    ConfigFile {
      name: other.name.or(self.name),
      profile: other.profile.or(self.profile),
      profiles: Profiles { profiles },
    }
  }
}

impl Default for ConfigFile {
  fn default() -> Self {
    ConfigFile {
      name: None,
      profile: None,
      profiles: Profiles::default(),
    }
  }
}

fn run(config: AppConfig) -> Result<(), Box<dyn Error>> {
  println!("Using profile: {}", config.profile_name);
  let chords_map = ChordsMap::new(config.profile.map.into_iter().map(|x| (x.key, x.val)).collect());
  let chordifier = Chordifier::new(chords_map);
  create_virtual_midi_device(config.name.as_str(), chordifier)
}

fn main() -> Result<(), Box<dyn Error>> {
  utils::common_inits::app_init()?;
  let cli = Cli::parse();
  run_cli_parsing(cli).and_then(run)
}

pub fn run_cli_parsing(cli: Cli) -> Result<AppConfig, Box<dyn Error>> {
  let Cli {
    config,
    name: _name,
    profile: _profile,
  } = cli;

  let default_config: ConfigFile =
    serde_yaml::from_str(include_str!("resources/default_config.yaml"))?;
  let user_config = if let Some(config_path) = config {
    let contents = read_to_string(config_path.clone())?;
    match config_path.extension() {
      Some(ext) if ext == "dhall" => {
        let yaml = utils::call_process::call_dhall_to_yaml(contents.into_bytes())
          .map_err(|e| format!("{:?}", e))?;
        serde_yaml::from_slice(&yaml)?
      }
      _ => serde_yaml::from_str(&contents)?,
    }
  } else {
    ConfigFile::default()
  };
  let cli_config = ConfigFile {
    name: _name,
    profile: _profile,
    profiles: Profiles::default(),
  };
  let config = default_config.merge(user_config).merge(cli_config);
  let profile_name = config.profile.unwrap_or("default".to_string());
  let profile = config.profiles.profiles.get(profile_name.as_str());
  match profile {
    Some(profile) => {
      let appconfig = AppConfig {
        name: config.name.unwrap_or("midi_mapper".to_string()),
        profile_name,
        profile: profile.clone(),
      };
      Ok(appconfig)
    }
    None => Err(format!("Profile {} not found", profile_name).into()),
  }
}

#[test]
fn verify_cli() {
  use clap::CommandFactory;
  Cli::command().debug_assert()
}
