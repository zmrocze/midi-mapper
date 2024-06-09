use fxhash::FxHashMap;
use itertools::Itertools;
use midly::{num::u7, MidiMessage};
use serde::Deserialize;

use crate::midi_device::MidiActionPassChannel;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chord {
  pub notes: Vec<Note>,
}

impl From<Vec<Note>> for Chord {
  fn from(notes: Vec<Note>) -> Self {
    Self { notes }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Note {
  pub note: u7,
}

impl<'a> Deserialize<'a> for Note {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>,
  {
    Deserialize::deserialize(deserializer).map(|x: u8| Note::new(x.into()))
  }
}

impl Note {
  pub fn new(note: u7) -> Self {
    Self { note }
  }
}

impl From<u7> for Note {
  fn from(note: u7) -> Self {
    Self { note }
  }
}

// #[derive(Clone, Debug)]
pub struct ChordsMap {
  mapping: FxHashMap<Chord, Chord>,
}

impl ChordsMap {
  pub fn new(mapping: Vec<(Chord, Chord)>) -> Self {
    // let m : FxHashMap<Chord, Chord> = HashMap::new();
    let mut m: FxHashMap<Chord, Chord> = FxHashMap::default();
    for (k, v) in mapping {
      let len = k.notes.len();
      for k1 in k.notes.into_iter().permutations(len) {
        m.insert(k1.into(), v.clone());
      }
    }
    Self { mapping: m }
  }

  pub fn map(&self, chord: &Chord) -> Option<&Chord> {
    self.mapping.get(chord)
  }
}

// Chord that is currently being played and pressed
struct PressedChord {
  chord: Chord,
  // playing : Chord
}

enum Response<'a> {
  SendChord(&'a Chord),
  Passthrough(MidiMessage),
  Received,
}

impl PressedChord {
  pub fn empty() -> Self {
    Self {
      chord: Chord { notes: Vec::new() },
    }
  }

  pub fn update<'a>(&mut self, mapping: &'a ChordsMap, note: MidiMessage) -> Response<'a> {
    use Response::*;
    // update pressed notes
    match note {
      MidiMessage::NoteOn { key, .. } => self.chord.notes.push(key.into()),
      MidiMessage::NoteOff { key, .. } => {
        for (i, x) in self.chord.notes.iter().enumerate() {
          if key == x.note {
            self.chord.notes.swap_remove(i);
            break;
          }
        }
      }
      other => return Passthrough(other),
    }
    // update playing notes
    if let Some(chord) = mapping.map(&self.chord) {
      // self.pressed.notes.clear();
      SendChord(chord)
    } else {
      Received
    }
  }
}

// TODO: maybe just make MidiAction take additional immutable ref thats passed from create_virtual_midi_device? interesting if compiler would treat this different then
// Pressed chord, playing chord and the static mapping - everything needed to implement `MidiAction`
// #[derive(Clone, Debug)]
pub struct Chordifier {
  // static mapping between chords
  mapping: ChordsMap,
  // currently pressed chord
  pressed: PressedChord,
  // currently playing chord
  playing: Option<Chord>,
}

impl Chordifier {
  pub fn new(mapping: ChordsMap) -> Self {
    Self {
      mapping,
      pressed: PressedChord::empty(),
      playing: None,
    }
  }
}

impl MidiActionPassChannel for Chordifier {
  // Maps chords to chords by tracking which chord is pressed and playing.
  // A chord plays only while the currently pressed chord corresponds to it.
  // NoteOn's and NoteOff's are send with max velocity (127)
  fn midi_action_on_msg<O>(&mut self, data: MidiMessage, mut outport: O)
  where
    O: FnMut(MidiMessage),
  {
    let mapping = &self.mapping;
    let pressed = &mut self.pressed;
    let playing = &mut self.playing;
    // The pressed chord changes in the case of SendChord and Received.
    let mut stop_playing = |outport: &mut O| match playing {
      Some(chord) => {
        for note in chord.notes.iter() {
          outport(MidiMessage::NoteOff {
            key: note.note,
            vel: u7::new(127),
          });
        }
        *playing = None;
      }
      None => (),
    };
    // Sends chords with max velocity!
    match pressed.update(mapping, data) {
      Response::SendChord(chord) => {
        stop_playing(&mut outport);
        for note in chord.notes.iter() {
          outport(MidiMessage::NoteOn {
            key: note.note,
            vel: u7::new(127),
          });
        }
        *playing = Some(chord.clone());
      }
      Response::Passthrough(msg) => outport(msg),
      Response::Received => stop_playing(&mut outport),
    }
  }
}
