use std::fmt;
use std::marker::PhantomData;

use fxhash::FxHashMap;
use itertools::Itertools;
use midly::num::u4;
use midly::{num::u7, MidiMessage};
use serde::de::{self};
use serde::Serialize;
use serde::{de::Visitor, Deserialize};

use crate::midi_device::{MidiAction, MidiData};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
pub struct ChannelChord {
  pub notes: Vec<ChannelNote>,
}

impl From<Vec<ChannelNote>> for ChannelChord {
  fn from(notes: Vec<ChannelNote>) -> Self {
    Self { notes }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Deserialize)]
pub struct ChannelNote {
  pub note: Note,
  pub channel: Channel,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Channel {
  pub channel: u4,
}

impl From<u4> for Channel {
  fn from(note: u4) -> Self {
    Channel::new(note)
  }
}

impl From<u8> for Channel {
  fn from(note: u8) -> Self {
    Channel::new(note.into())
  }
}

impl<'a> Deserialize<'a> for Channel {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>,
  {
    // Deserialize::deserialize(deserializer).map(|x: u8| Channel::new(x.into()))
    deserializer.deserialize_any(NoteVisitor { _a: PhantomData::<Channel> })
  }
}

impl Channel {
  pub fn new(channel: u4) -> Self {
    Self { channel }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Note {
  pub note: u7,
}

impl From<u8> for Note {
  fn from(note: u8) -> Self {
    Note::new(note.into())
  }
}

impl From<u7> for Note {
  fn from(note: u7) -> Self {
    Self { note }
  }
}

impl<'a> Deserialize<'a> for Note {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'a>,
  {
    // Deserialize::deserialize(deserializer).map(|x: u8| Note::new(x.into()))
    deserializer.deserialize_any(NoteVisitor { _a: PhantomData::<Note> })
  }
}

impl Note {
  pub fn new(note: u7) -> Self {
    Self { note }
  }
}

struct NoteVisitor<A> { _a: std::marker::PhantomData<A> }

impl<'de, A : From<u8>> Visitor<'de> for NoteVisitor<A> {
  type Value = A;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("an integer between 0 and 127 or a string with such integer")
  }

  fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Ok(value.into())
  }

  fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let n = v.parse::<u8>().map_err(de::Error::custom)?;
    Ok(n.into())
  }
  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let n = v.parse::<u8>().map_err(de::Error::custom)?;
    Ok(n.into())
  }

  fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let n = u8::try_from(value).map_err(de::Error::custom)?;
    Ok(n.into())
  }
  fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let n = u8::try_from(value).map_err(de::Error::custom)?;
    Ok(n.into())
  }
  fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let n = u8::try_from(value).map_err(de::Error::custom)?;
    Ok(n.into())
  }
  fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let n = u8::try_from(value).map_err(de::Error::custom)?;
    Ok(n.into())
  }
  fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let n = u8::try_from(v).map_err(de::Error::custom)?;
    Ok(n.into())
  }
  fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let n = u8::try_from(v).map_err(de::Error::custom)?;
    Ok(n.into())
  }
}

// #[derive(Clone, Debug)]
pub struct ChordsMap {
  mapping: FxHashMap<ChannelChord, ChannelChord>,
}

impl ChordsMap {
  pub fn new(mapping: Vec<(ChannelChord, ChannelChord)>) -> Self {
    // let m : FxHashMap<ChannelChord, ChannelChord> = HashMap::new();
    let mut m: FxHashMap<ChannelChord, ChannelChord> = FxHashMap::default();
    for (k, v) in mapping {
      let len = k.notes.len();
      for k1 in k.notes.into_iter().permutations(len) {
        m.insert(k1.into(), v.clone());
      }
    }
    Self { mapping: m }
  }

  pub fn map(&self, channelChord: &ChannelChord) -> Option<&ChannelChord> {
    self.mapping.get(channelChord)
  }
}

// ChannelChord that is currently being played and pressed
struct PressedChord {
  chord: ChannelChord,
  // playing : ChannelChord
}

enum Response<'a> {
  SendChord(&'a ChannelChord),
  Passthrough(MidiData),
  Received,
}

impl PressedChord {
  pub fn empty() -> Self {
    Self {
      chord: ChannelChord { notes: Vec::new() },
    }
  }

  pub fn update<'a>(&mut self, mapping: &'a ChordsMap, note: MidiData) -> Response<'a> {
    use Response::*;
    // update pressed notes
    let MidiData { channel, message } = note;
    match message {
      MidiMessage::NoteOn { key, .. } => self.chord.notes.push( ChannelNote { channel : Channel::new(channel), note: key.into() }),
      MidiMessage::NoteOff { key, .. } => {
        for (i, x) in self.chord.notes.iter().enumerate() {
          if key == x.note.note && channel == x.channel.channel {
            self.chord.notes.swap_remove(i);
            break;
          }
        }
      }
      other => return Passthrough(note.clone()),
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
// Pressed ChannelChord, playing ChannelChord and the static mapping - everything needed to implement `MidiAction`
// #[derive(Clone, Debug)]
pub struct Chordifier {
  // static mapping between chords
  mapping: ChordsMap,
  // currently pressed ChannelChord
  pressed: PressedChord,
  // currently playing ChannelChord
  playing: Option<ChannelChord>,
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

impl MidiAction for Chordifier {
  // Maps chords to chords by tracking which ChannelChord is pressed and playing.
  // A ChannelChord plays only while the currently pressed ChannelChord corresponds to it.
  // NoteOn's and NoteOff's are send with max velocity (127)
  fn midi_action<O>(&mut self, data: MidiData, mut outport: O)
  where
    O: FnMut(MidiData),
  {
    let mapping = &self.mapping;
    let pressed = &mut self.pressed;
    let playing = &mut self.playing;
    // The pressed ChannelChord changes in the case of SendChord and Received.
    let mut stop_playing = |outport: &mut O| match playing {
      Some(chord) => {
        for note in chord.notes.iter() {
          outport(MidiData {
              message : MidiMessage::NoteOff {
                key: note.note.note,
                vel: u7::new(127),
              },
              channel: note.channel.channel,
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
          outport(MidiData { 
            channel: note.channel.channel,
            message: MidiMessage::NoteOn {
              key: note.note.note,
              vel: u7::new(127),
            }
          });
        }
        *playing = Some(chord.clone());
      }
      Response::Passthrough(msg) => outport(msg),
      Response::Received => stop_playing(&mut outport),
    }
  }
}
