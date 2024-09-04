use std::collections::HashMap;

use midly::num::u7;
use serde::Deserialize;

use crate::chordifier::{Chord, ChordsMap, Note};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Deserialize)]
pub enum ChordType {
  MAJ,
  MIN,
  DIM,
  AUG,
  DOM7,
  MIN7,
  MAJ7,
  MINMAJ7,
  DIM7,
  HDIM7,
  AUGMAJ7,
}

impl ChordType {
  fn intervals(&self) -> Vec<u7> {
    match self {
      ChordType::MAJ => vec![0.into(), 4.into(), 7.into()],
      ChordType::MIN => vec![0.into(), 3.into(), 7.into()],
      ChordType::DIM => vec![0.into(), 3.into(), 6.into()],
      ChordType::AUG => vec![0.into(), 4.into(), 8.into()],
      ChordType::DOM7 => vec![0.into(), 4.into(), 7.into(), 10.into()],
      ChordType::MIN7 => vec![0.into(), 3.into(), 7.into(), 10.into()],
      ChordType::MAJ7 => vec![0.into(), 4.into(), 7.into(), 11.into()],
      ChordType::MINMAJ7 => vec![0.into(), 3.into(), 7.into(), 11.into()],
      ChordType::DIM7 => vec![0.into(), 3.into(), 6.into(), 9.into()],
      ChordType::HDIM7 => vec![0.into(), 3.into(), 6.into(), 10.into()],
      ChordType::AUGMAJ7 => vec![0.into(), 4.into(), 8.into(), 11.into()],
    }
  }
}

struct ChordByType {
  root: Note,
  chord_type: ChordType,
}

impl From<ChordByType> for Chord {
  fn from(value: ChordByType) -> Self {
    let notes: Vec<Note> = value
      .chord_type
      .intervals()
      .into_iter()
      .map(|x| Note::new(x + value.root.note))
      .collect();
    Chord { notes }
  }
}

pub fn make_mapping(
  roots: HashMap<Note, Note>,
  chord_types: HashMap<Note, ChordType>,
) -> ChordsMap {
  let mut map: Vec<(Chord, Chord)> = Vec::new();
  for (note1, root) in roots {
    for (note2, chord_type) in &chord_types {
      let chord = (ChordByType {
        root,
        chord_type: chord_type.clone(),
      })
      .into();
      map.push((vec![note1, *note2].into(), chord));
    }
  }
  ChordsMap::new(map)
}
