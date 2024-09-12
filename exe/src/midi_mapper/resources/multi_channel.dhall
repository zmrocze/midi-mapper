
let 
  lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
in { profiles.default.map
  =
  let config =
    {
      roots = lib.direct_mapped_roots (lib.note-range { channel = +0, from = +30, to = +50 }),
      intervals = [
        lib.played_on_channels [+0] (lib.direct_mapped_intervals (lib.note-range { channel = +1, from = +0, to = +12 })),
        lib.played_on_channels [+0] (lib.direct_mapped_intervals (lib.note-range { channel = +2, from = -12, to = +12 }))
      ]
    }
  in lib.by_intervals config
}