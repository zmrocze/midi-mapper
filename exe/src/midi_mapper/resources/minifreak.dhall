
let 
  lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
in { profiles = {
    minifreak_1 = {
      map = (
        let config = {
          roots =
            let Root = { root : Integer, intervals : List lib.Note }
            let RootPair = lib.Pair lib.Note Root
            in
              lib.list-map lib.Note RootPair
                (\(note : lib.Note) -> 
                  { key = note, 
                    val = { root = note.note , intervals = [ { note = +0, channel = note.channel } ] }}
                )
                lib.minifreak
            ,

          intervals =
              let mkIntervals = \(arg : { pressed_channel : Integer, played_channel : Integer }) -> 
                  lib.played_on_channels [ arg.played_channel ] (lib.middle_mapped_intervals
                    (lib.note-range { channel = arg.pressed_channel, from = -9, to = +9 }))
              in
                [
                  mkIntervals { pressed_channel = +1, played_channel = +0 },
                  mkIntervals { pressed_channel = +1, played_channel = +0 },
                  -- mkIntervals { pressed_channel = +1, played_channel = +1 }
                ]
          }
        in lib.by_intervals config)
      }
      ,
    minifreak_2.map =
      -- as of nov 2024 there's 11 chords '-'
      let chord_types = lib.zip-pairs Integer lib.ChordTypes (lib.range {from = +72, to = +84 }) lib.all_chord_types
      let first2octaves = lib.range {from = +48, to = +72 }
      let roots = lib.zip-pairs Integer Integer first2octaves first2octaves
    in lib.by_chord_type { chord_types = chord_types, roots = roots }
  }
}