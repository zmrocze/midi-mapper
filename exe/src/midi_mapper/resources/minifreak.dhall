
let lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
let minifreak_first_2_octaves = lib.range {from = +48, to = +72 }
let minifreak_third_octave = lib.range {from = +72, to = +84 }
in { 
  profiles = {
    minifreak_w_tidal_intervals = {
      map = (
        let config = {
          roots = lib.direct_mapped_roots lib.minifreak,

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
      },
    minifreak_chordtypes = {
      map =
          (let chord_types = lib.zip-pairs Integer lib.ChordTypes minifreak_third_octave lib.all_chord_types
          let roots = lib.zip-pairs Integer Integer minifreak_first_2_octaves minifreak_first_2_octaves
          in lib.by_chord_type { chord_types = chord_types, roots = roots }),
      },
    minifreak_march25 = {
        map =
          let some_intervals = [
            [ +0, +10, +17 ],         -- 1 * blueA
            [ +0, +14 ],              -- 2 * blueB
            [ +0, +7, +18, +21],      -- 3 * (blueC)
            [ +0, +3, +7, +9 ],       -- 4
            [ +0, +7, +22 ],          -- 5 * (yellow)
            [ +0, +4, +7, +12 ],      -- 6
            [ +0, +7, +10, +19, +27 ], -- 7 * (greenA)
            [ +0, +3, +12, +21, +30 ], -- 8 * (breenB)
            [ +0, +3, +12, +24 ],      -- 9 * (breenA)
            [ +0, +5, +16 ],           -- 10 * (greenB)
            [+0, +5, +8, +15, +20],    -- 11 * greyA
            [+0, +4, +9, +12]          -- 12 * greyB
          ]
          in lib.by_chord_intervals { 
            roots = lib.zip-pairs Integer Integer minifreak_first_2_octaves minifreak_first_2_octaves,
            chord_types = (lib.zip-pairs Integer (List Integer) minifreak_third_octave some_intervals)
          }
        },
    -- equivalent to march25 but try using the other interface to check
    minifreak_march25_2 = {
        map =
          let some_intervals = lib.list-map (List Integer) { intervals : List lib.Note } 
            (\(intervals : List Integer) -> { intervals = lib.on-channel { channel = +0, notes = intervals}}) 
            [
              [ +0, +10, +17 ],         -- 1 * blueA
              [ +0, +14 ],              -- 2 * blueB
              [ +0, +7, +18, +21],      -- 3 * (blueC)
              [ +0, +3, +7, +9 ],       -- 4
              [ +0, +7, +22 ],          -- 5 * (yellow)
              [ +0, +4, +7, +12 ],      -- 6
              [ +0, +7, +10, +19, +27 ], -- 7 * (greenA)
              [ +0, +3, +12, +21, +30 ], -- 8 * (breenB)
              [ +0, +3, +12, +24 ],      -- 9 * (breenA)
              [ +0, +5, +16 ],           -- 10 * (greenB)
              [+0, +5, +8, +15, +20],    -- 11 * greyA
              [+0, +4, +9, +12]          -- 12 * greyB
            ]
          in lib.by_intervals {
            roots = lib.direct_mapped_roots (lib.on-channel { channel = +0, notes = minifreak_first_2_octaves }),
            intervals = [
              (lib.zip-pairs lib.Note { intervals : List lib.Note } (lib.on-channel { channel = +0, notes = minifreak_third_octave}) some_intervals)
            ]
          }
        }
  }
}