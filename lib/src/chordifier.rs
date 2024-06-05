// use std::collections::HashMap;

use std::collections::HashMap;

use fxhash::FxHashMap;
use itertools::Itertools;
use midly::{live::LiveEvent, num::{u4, u7}, MidiMessage};
use serde::Deserialize;

use crate::midi_device::MidiActionPassChannel;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chord { pub notes: Vec<Note> }

impl From<Vec<Note>> for Chord {
  fn from(notes : Vec<Note>) -> Self {
    Self { notes }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Note { pub note : u7 }

impl<'a> Deserialize<'a> for Note {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
          D: serde::Deserializer<'a> {
    Deserialize::deserialize(deserializer).map(|x: u8| Note::new(x.into()))
  }
}

impl Note {
  pub fn new(note : u7) -> Self {
    Self { note }
  }
}

impl From<u7> for Note {
  fn from(note : u7) -> Self {
    Self { note }
  }
}

// #[derive(Clone, Debug)]
pub struct ChordsMap {
  mapping : FxHashMap<Chord, Chord>
}

impl ChordsMap {
  pub fn new(mapping : Vec<(Chord, Chord)>) -> Self {
    // let m : FxHashMap<Chord, Chord> = HashMap::new(); 
    let mut m : FxHashMap<Chord, Chord> = FxHashMap::default();
    for (k, v) in mapping {
      let len = k.notes.len();
      for k1 in k.notes.into_iter().permutations(len) {
        m.insert(k1.into(), v.clone());
      }
    }
    Self { mapping : m }
  }

  pub fn map(&self, chord : &Chord) -> Option<&Chord> {
    self.mapping.get(chord)
  }
}

// Chord that is currently being played
struct ChordOn {
  chord : Chord
}

enum Response<'a> {
  SendChord(&'a Chord),
  Passthrough(MidiMessage),
  Received
}

impl ChordOn {
  pub fn new(chord : Chord) -> Self {
    Self { chord }
  }

  pub fn update<'a>(&mut self, mapping: &'a ChordsMap, note: MidiMessage) -> Response<'a> {
    use Response::*;
    match note {
      MidiMessage::NoteOn { key, .. } => self.chord.notes.push(key.into()),
      MidiMessage::NoteOff { key, .. } => {
        for (i, x) in self.chord.notes.iter().enumerate() {
          if key == x.note {
            self.chord.notes.swap_remove(i);
            break;
          }
        }
      },
      other => return Passthrough(other)
    }
    if let Some(chord) = mapping.map(&self.chord) {
      self.chord.notes.clear();
      SendChord(chord)
    } else {
      Received
    }
  }
}

// TODO: maybe just make MidiAction take additional immutable ref thats passed from create_virtual_midi_device? interesting if compiler would treat this different then
// Pressed chord and the static mapping - everything needed to implement `MidiAction`
// #[derive(Clone, Debug)]
pub struct Chordifier {
  mapping : ChordsMap,
  chord_on : ChordOn
}

impl Chordifier {
  pub fn new(mapping : ChordsMap) -> Self {
    Self { mapping, chord_on : ChordOn::new(Chord { notes: Vec::new() }) }
  }
}

impl MidiActionPassChannel for Chordifier { 
  fn midi_action_on_msg<O>(&mut self, data : MidiMessage, mut outport : O ) where 
    O: FnMut(MidiMessage) {
    // Sends chords with max velocity!
    match self.chord_on.update(&self.mapping, data) {
      Response::SendChord(chord) => {
        for note in chord.notes.iter() {
          outport(MidiMessage::NoteOn { key: note.note, vel: u7::new(127) });
        }
      },
      Response::Passthrough(msg) => outport(msg),
      Response::Received => ()
    }
  }
}

// #[macro_use]
// extern crate lazy_static;

// use std::collections::HashMap;

// lazy_static! {
//     static ref HASHMAP: HashMap<u32, &'static str> = {
//         let mut m = HashMap::new();
//         m.insert(0, "foo");
//         m.insert(1, "bar");
//         m.insert(2, "baz");
//         m
//     };
// }