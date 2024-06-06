use serde::Deserialize;
use std::error::Error;
use clap::Parser;
use midi_mapper::midi_device::{create_virtual_midi_device, MidiAction};

#[derive(Parser, Debug, Deserialize)]
#[command(author, version, about, long_about = "A virtual device that prints incoming MIDI messages and passes them through.")]
pub struct Cli {
}

struct Printer {}

impl MidiAction for Printer {
  fn midi_action<O>(&mut self, data : midi_mapper::midi_device::MidiData, mut outport : O ) where 
      O: FnMut(midi_mapper::midi_device::MidiData) {
    println!("received: {:?}", data);
    outport(data);
  }
}


fn run() -> Result<(), Box<dyn Error>> {
  create_virtual_midi_device(
    "midi-printer",
    Printer {}
  )
}

fn main() -> Result<(), Box<dyn Error>> {
  utils::common_inits::app_init()?;
  let Cli { } = Cli::parse();
  run()
}

#[test]
fn verify_cli() {
  use clap::CommandFactory;
  Cli::command().debug_assert()
}
