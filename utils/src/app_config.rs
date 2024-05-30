use config::{Config, Environment};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::RwLock;
use lazy_static::lazy_static;
use serde::Deserialize;

use super::error::Result;

struct CliConfig {
  config: Config
}

impl CliConfig {
    pub fn init(default_config: Option<&str>, env_prefix : &str) -> Result<Self> {
        let mut settings = Config::new();

        // Embed file into executable
        // This macro will embed the configuration file into the
        // executable. Check include_str! for more info.
        if let Some(config_contents) = default_config {
            //let contents = include_str!(config_file_path);
            settings.merge(config::File::from_str(&config_contents, config::FileFormat::Toml))?;
        }

        // Merge settings with env variables
        settings.merge(Environment::with_prefix(env_prefix))?;

        // TODO: Merge settings with Clap Settings Arguments

        // Save Config to RwLoc
        // {
        //     let mut w = CONFIG.write()?;
        //     *w = settings;
        // }

        Ok(CliConfig { config = settings })
    }

    pub fn merge_config(&mut self, config_file: Option<PathBuf>) -> Result<()> {
        // Merge settings with config file if there is one
        if let Some(config_file_path) = config_file {
            let config = self.config.merge(config::File::from(config_file_path))?;
            self.config = config;
            Ok(())
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
        Ok(self.config.read()?.get::<T>(key)?)
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
}

impl<TryInto<Config, A>> TryInto<CliConfig, A> for CliConfig {
    type Error = TryInto<Config,A>::Error;

    fn try_into(self) -> Result<A, Error> {
        self.config.try_into()
    }
}
