let list-map = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/map.dhall
let concat-map = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/concatMap.dhall
let int-add = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/Integer/add.dhall

-- types
let
  ChordTypes = <
    MAJ
    | MIN
    | DIM
    | AUG
    | DOM7
    | MIN7
    | MAJ7
    | MINMAJ7
    | DIM7
    | HDIM7
    | AUGMAJ7
  >
let Note = { note : Integer, channel : Integer }
let 
  IntInt = { key : Integer, val : Integer}
let 
  IntChordType = { key : Integer, val : ChordTypes}
let Chord = { notes : List Note }
let chord = \(notes : List Note) -> { notes = notes }
let ChordPair = { key : Chord, val : Chord }

let intervals = \(type : ChordTypes) -> merge {
    MAJ = [+0, +4, +7],
    MIN = [+0, +3, +7],
    DIM = [+0, +3, +6],
    AUG = [+0, +4, +8],
    DOM7 = [+0, +4, +7, +10],
    MIN7 = [+0, +3, +7, +10],
    MAJ7 = [+0, +4, +7, +11],
    MINMAJ7 = [+0, +3, +7, +11],
    DIM7 = [+0, +3, +6, +9],
    HDIM7 = [+0, +3, +6, +10],
    AUGMAJ7 = [+0, +4, +8, +11],
  } type

-- let ChordsMap = List ChordPair
let channel-zero = \(n : Integer) -> { note = n , channel = +0 }
let
  -- using midi channel 0 
  by_chord_type = \(arg : { 
    roots : List IntInt, 
    chord_types : List IntChordType
  } ) ->
    concat-map IntInt ChordPair ( \(r: IntInt) -> 
      list-map IntChordType ChordPair ( \(ch : IntChordType) -> 
        {
          key = chord [channel-zero r.key, channel-zero ch.key], 
          val = chord (list-map Integer Note (\(x: Integer) ->  channel-zero (int-add x r.val)) (intervals ch.val))
        }
      )
      arg.chord_types
    ) arg.roots
  in {
  by_chord_type = by_chord_type,
  ChordTypes = ChordTypes
}