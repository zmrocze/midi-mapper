let 
  lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
in { profiles.default.map
  =
  let chord_types =
    [ { key = +53, val = lib.ChordTypes.MAJ }
    , { key = +55, val = lib.ChordTypes.MIN }
    , { key = +57, val = lib.ChordTypes.DIM }
    , { key = +59, val = lib.ChordTypes.AUG }
    , { key = +60, val = lib.ChordTypes.DOM7 }
    , { key = +62, val = lib.ChordTypes.MIN7 }
    , { key = +64, val = lib.ChordTypes.MAJ7 }
    , { key = +65, val = lib.ChordTypes.MINMAJ7 }
    , { key = +67, val = lib.ChordTypes.DIM7 }
    , { key = +69, val = lib.ChordTypes.HDIM7 }
    , { key = +71, val = lib.ChordTypes.AUGMAJ7 }
    ]

  let roots =
    [ { key = +36, val = +48 }
    , { key = +37, val = +49 }
    , { key = +38, val = +50 }
    , { key = +39, val = +51 }
    , { key = +40, val = +52 }
    , { key = +41, val = +53 }
    , { key = +42, val = +54 }
    , { key = +43, val = +55 }
    , { key = +44, val = +56 }
    , { key = +45, val = +57 }
    , { key = +46, val = +58 }
    , { key = +47, val = +59 }
    , { key = +48, val = +60 }
    , { key = +49, val = +61 }
    , { key = +50, val = +62 }
    , { key = +51, val = +63 }
    , { key = +52, val = +64 }
    ]

  in lib.by_chord_type { chord_types = chord_types, roots = roots }
}