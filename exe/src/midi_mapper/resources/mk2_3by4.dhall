
let 
  lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
in { profiles.mk2_3by4 = {
  setting_note_0vel_is_noteoff = True,
  map = (
    let
      mk2_used_indices = lib.list-concat Integer [
        -- first octave
        lib.range { from = +0, to = +4 },
        lib.range { from = +8, to = +12 },
        lib.range { from = +16, to = +20 },
        -- second octave
        lib.range { from = +24, to = +28 },
        lib.range { from = +32, to = +36 },
        lib.range { from = +40, to = +44 },
        -- third octave
        lib.range { from = +48, to = +52 },
        lib.range { from = +56, to = +60 },
        lib.range { from = +64, to = +68 },
      ]
    let mk2_keys = lib.select_from_keyboard { indices = mk2_used_indices, keys = lib.novation-mk2 }
    let config = {
        roots =
          let Root = { root : Integer, intervals : List lib.Note }
          let RootPair = lib.Pair lib.Note Root
          in
            lib.list-map (lib.Pair lib.Note lib.Note) RootPair
              (\(pr : lib.Pair lib.Note lib.Note) -> 
                { key = pr.key, 
                  val = { root = pr.val.note , intervals = [ { note = +0, channel = pr.val.channel } ] } }
              )
              (lib.create_contingent_mapping { keys = mk2_keys, channel = +0, from = +50 })
        ,

        intervals =
          let mkIntervals = \(arg : { pressed_channel : Integer, played_channel : Integer  }) -> 
            lib.list-map Integer (lib.Pair lib.Note { intervals : List lib.Note})
              (\(i : Integer) -> { 
                key = { note = lib.int-add +60 i, channel = arg.pressed_channel },
                val = { intervals = [ { note = i, channel = arg.played_channel } ] }
              })
              (lib.range { from = -9, to = +4 })
          in
            [
              mkIntervals { pressed_channel = +0, played_channel = +0 },
              mkIntervals { pressed_channel = +1, played_channel = +1 }
            ]
      }
    in lib.by_intervals config)
  }
}