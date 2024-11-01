let 
  lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
in { profiles.simple.map
  =
  let chord_types =
    [ { key = +53, val = lib.ChordTypes.MAJ }
    ]

  let roots =
    [ { key = +36, val = +48 }
    ]

  in lib.by_chord_type { chord_types = chord_types, roots = roots }
}