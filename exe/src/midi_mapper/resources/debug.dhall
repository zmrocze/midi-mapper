
let 
  lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
let 
  minifreak = lib.note-range { channel = +0, from = +48, to = +84 }
in { profiles.debug = {
  setting_note_0vel_is_noteoff = True,
  map = (
    let
      mk2_used_indices = lib.list-concat Integer [
        -- first octave
        lib.range { from = +0, to = +2 },
      ]
    let config = {
        roots =
          let Root = { root : Integer, intervals : List lib.Note }
          let RootPair = lib.Pair lib.Note Root
          in
            lib.list-map lib.Note RootPair
              (\(note : lib.Note) -> 
                { 
                  key = note, 
                  val = { root = note.note , intervals = [ { note = +0, channel = note.channel } ] }
                }
              )
              minifreak
        ,

        intervals =
          let mkIntervals = \(arg : { pressed_channel : Integer, played_channel : Integer  }) -> 
            lib.list-map Integer (lib.Pair lib.Note { intervals : List lib.Note})
              (\(i : Integer) -> {
                key = { note = lib.int-add +60 i, channel = arg.pressed_channel },
                val = { intervals = [ { note = i, channel = arg.played_channel } ] }
              })
              (lib.range { from = -1, to = +1 })
          in
            [
              mkIntervals { pressed_channel = +1, played_channel = +0 }
              -- , mkIntervals { pressed_channel = +3, played_channel = +4 }
            ]
      }
    in lib.by_intervals config)
  }
}