#[cfg(test)]
mod tests {
  use std::{path::PathBuf, str::FromStr};

  use chordifier::midi_mapper::main::{run_cli_parsing, Cli, ConfigFile, Profiles};

  macro_rules! test_case {
    ($fname:expr) => {{
      let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../exe/src/midi_mapper/resources/",
        $fname
      ); // assumes Linux ('/')!
      PathBuf::from_str((path).to_string().as_str()).unwrap()
    }};
  }

  #[test]
  fn parse_config_default_dhall() {
    let path = test_case!("default_config.dhall");
    let cli = Cli {
      config: Some(path.clone()),
      name: Some("sone test name".to_string()),
      profile: None,
    };
    let r = run_cli_parsing(cli);
    assert!(
      r.is_ok(),
      "parse_config_default_dhall failed {:?} {:?}",
      r,
      path
    );
  }

  #[test]
  fn parse_config_default_yaml() {
    let path = test_case!("default_config.yaml");
    let cli = Cli {
      config: Some(path),
      name: Some("sone test name".to_string()),
      profile: None,
    };
    let r = run_cli_parsing(cli);
    assert!(r.is_ok(), "parse_config_default_yaml failed {:?}", r);
  }

  #[test]
  fn parse_config_simple_dhall() {
    let path = test_case!("simple_config.dhall");
    let cli = Cli {
      config: Some(path),
      name: Some("sone test name".to_string()),
      profile: None,
    };
    let r = run_cli_parsing(cli);
    assert!(r.is_ok(), "parse_config_simple_dhall failed {:?}", r);
  }
}
