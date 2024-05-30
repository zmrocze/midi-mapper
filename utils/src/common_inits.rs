
fn common_inits(default_config_path : &str, env_prefix : &str) -> Result<CliConfig> {
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
  let config_contents = include_str!(default_config_path);
  let config = CliConfig::init(Some(config_contents), env_prefix)?;

  // Setup Logging
  install_logger()?

  Ok(config)

}
