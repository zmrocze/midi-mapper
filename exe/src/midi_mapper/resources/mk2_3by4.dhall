
let 
  lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
in { profiles.default.map
  =
  let
    mk2_used_indices = lib.list-concat Integer [
      -- first octave
      lib.range { from = +0, to = +4 },
      lib.range { from = +8, to = +12 },
      lib.range { from = +16, to = +20 },
      -- second octave
      lib.range { from = +24, to = +28 },
      lib.range { from = +32, to = +36 },
      lib.range { from = +40, to = +44 }
    ]
  let n = List/length Integer mk2_used_indices
  let config =
    {
      roots =
        let Root = { root : Integer, intervals : List lib.Note }
        let RootPair = lib.Pair lib.Note Root
        in
          lib.list-map (lib.Pair lib.Note RootPair) RootPair
            (\(pr : lib.Pair lib.Note RootPair) -> { key = pr.key, val = pr.val.val })
            (lib.zip-pairs lib.Note RootPair
              -- trigger notes, defined as indices in novation-mk2 list of notes
              (lib.list-map Integer lib.Note
                (\(i : Integer) ->
                  lib.optional-default lib.Note {note = +0, channel=+0}
                    (lib.list-index (Integer/clamp i) lib.Note lib.novation-mk2)
                ) 
                mk2_used_indices
              )
              -- played notes, from 30 to 30+n
              (lib.direct_mapped_roots (lib.note-range { channel = +0, from = +30, to = lib.int-add +30 (Natural/toInteger n) }))
            )
      ,

      intervals = [
        (lib.played_on_channels [+1] (lib.direct_mapped_intervals (lib.note-range { channel = +1, from = +0, to = +12 })))
      ]
    }
  in lib.by_intervals config
}