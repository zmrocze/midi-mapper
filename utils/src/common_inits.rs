#[cfg(not(debug_assertions))]
use human_panic::setup_panic;

#[cfg(debug_assertions)]
extern crate better_panic;

use crate::cli_config::CliConfig;
use crate::error::Result;
use crate::logger::install_logger;

pub fn common_inits(config_contents: &str, env_prefix: &str) -> Result<CliConfig> {
  // Human Panic. Only enabled when *not* debugging.
  #[cfg(not(debug_assertions))]
  {
    setup_panic!();
  }

  // Better Panic. Only enabled *when* debugging.
  #[cfg(debug_assertions)]
  {
    better_panic::Settings::debug()
      .most_recent_first(false)
      .lineno_suffix(true)
      .verbosity(better_panic::Verbosity::Full)
      .install();
  }

  // Initialize Configuration
  let config = CliConfig::init(Some(config_contents), env_prefix)?;

  // Setup Logging
  install_logger()?;

  Ok(config)
}

pub fn app_init() -> Result<()> {
  // Human Panic. Only enabled when *not* debugging.
  #[cfg(not(debug_assertions))]
  {
    setup_panic!();
  }

  // Better Panic. Only enabled *when* debugging.
  #[cfg(debug_assertions)]
  {
    better_panic::Settings::debug()
      .most_recent_first(false)
      .lineno_suffix(true)
      .verbosity(better_panic::Verbosity::Full)
      .install();
  }

  // Setup Logging
  install_logger()?;

  Ok(())
}
