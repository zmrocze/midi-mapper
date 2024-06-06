use config::{Config, Environment};
use serde::Deserialize;
use std::path::PathBuf;

use super::error::Result;

#[derive(Debug)]
pub struct CliConfig {
  pub config: Config,
}

impl CliConfig {
  pub fn init(default_config: Option<&str>, env_prefix: &str) -> Result<Self> {
    let mut settings = Config::new();

    // Embed file into executable
    // This macro will embed the configuration file into the
    // executable. Check include_str! for more info.
    if let Some(config_contents) = default_config {
      //let contents = include_str!(config_file_path);
      settings.merge(config::File::from_str(
        &config_contents,
        config::FileFormat::Yaml,
      ))?;
    }

    // Merge settings with env variables
    settings.merge(Environment::with_prefix(env_prefix))?;

    // TODO: Merge settings with Clap Settings Arguments

    // Save Config to RwLoc
    // {
    //     let mut w = CONFIG.write()?;
    //     *w = settings;
    // }

    Ok(CliConfig { config: settings })
  }

  pub fn merge_config(&mut self, config_file: Option<PathBuf>) -> Result<()> {
    // Merge settings with config file if there is one
    if let Some(config_file_path) = config_file {
      self.config.merge(config::File::from(config_file_path))?;
    }
    Ok(())
  }

  // Set CONFIG
  pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
    {
      // Set Property
      self.config.set(key, value)?;
    }

    Ok(())
  }

  // Get a single value
  pub fn get<'de, T>(&self, key: &'de str) -> Result<T>
  where
    T: serde::Deserialize<'de>,
  {
    Ok(self.config.get::<T>(key)?)
  }

  // Get CONFIG
  // This clones Config (from RwLock<Config>) into a new AppConfig object.
  // This means you have to fetch this again if you changed the configuration.
  // pub fn fetch() -> Result<AppConfig> {
  //     // Get a Read Lock from RwLock
  //     let r = CONFIG.read()?;

  //     // Clone the Config object
  //     let config_clone = r.deref().clone();

  //     // Coerce Config into AppConfig
  //     Ok(config_clone.try_into()?)
  // }
  pub fn fetch<'de, A>(self) -> Result<A>
  where
    A: Deserialize<'de>,
  {
    let a = self.config.try_into()?;
    Ok(a)
  }
}

// impl< A : TryFrom<Config>> TryFrom<CliConfig> for A {
//     type Error = <A as TryFrom<Config>>::Error;

//     fn try_from(value: CliConfig) -> std::prelude::v1::Result<Self, Self::Error> {
//     value.config.try_into()
//     }
// }
