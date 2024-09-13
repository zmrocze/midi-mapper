let list-map = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/map.dhall
let concat-map = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/concatMap.dhall
let int-add = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/Integer/add.dhall
let int-mul = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/Integer/multiply.dhall
let int-eq = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/Integer/equal.dhall
let int-sub = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/Integer/subtract.dhall
let list-concat = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/concat.dhall
let list-empty = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/empty.dhall
let list-filter = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/filter.dhall
let optional-null = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/Optional/null.dhall
let optional-default = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/Optional/default.dhall
let list-unzip = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/unzip.dhall
let list-cons = \(a : Type) -> \(x : a) -> \(xs : List a) -> list-concat a [ [ x ] , xs ]
let list-mapWithIndex = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/mapWithIndex.dhall
let list-zip = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/zip.dhall
let list-index = https://raw.githubusercontent.com/dhall-lang/dhall-lang/v23.0.0/Prelude/List/index.dhall
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
let Pair = \(l : Type) -> \(r : Type) -> { key : l, val : r }
let unzip-pairs = \(l : Type) -> \(r : Type) -> \(xs : List (Pair l r)) ->
  let yy = list-unzip l r (
    list-map (Pair l r) { _1 : l, _2 : r } (\(x : Pair l r) -> { _1 = x.key, _2 = x.val }) xs
  )
  in { key = yy._1, val = yy._2 }
let zip-pairs = \(l : Type) -> \(r : Type) -> \(ls : List l) -> \(rs : List r) ->
  let yy = list-zip l ls r rs
  in (
    list-map { _1 : l, _2 : r } (Pair l r) (\(x : { _1 : l, _2 : r }) -> { key = x._1, val = x._2 }) yy
  )
let Mapping = \(l : Type) -> \(r : Type) -> List ( Pair l r )
-- let NotePair = Pair Note Note
let 
  IntInt = Pair Integer Integer
let 
  IntChordType = Pair Integer ChordTypes
let Chord = { notes : List Note }
let chord = \(notes : List Note) -> { notes = notes }
let ChordPair = Pair Chord Chord
let Configuration = Mapping Chord Chord

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
let
  range = \(range : {from : Integer, to : Integer }) -> 
    List/build Integer (\(list : Type) -> \(cons : (Integer -> list -> list)) -> \(nil : list) -> 
      let
        len = Integer/clamp (int-sub range.from range.to)
      let 
        acc = Natural/fold len { k : Integer, xs : list }
          (\(acc : { k : Integer, xs : list }) -> { k = int-add acc.k +1, xs = cons acc.k acc.xs })
          { k = range.from, xs = nil }
      in acc.xs
    )
let
  note-range = \(arg : { channel : Integer, from : Integer, to : Integer }) ->
    list-map Integer Note (\(x: Integer) -> { note = x, channel = arg.channel }) (range { from = arg.from, to = arg.to })
let
  cross-product = \(a : Type ) -> \(xxs : List (List a)) ->
    (List/fold (List a) xxs (List (List a))
      (\(xs : List a) -> \(acc : (List (List a))) -> 
        (concat-map a (List a)
          (\(x : a) -> list-map (List a) (List a) (\(y : List a) -> list-cons a x y) acc )
          xs
        )
      )
      ([ (list-empty a) ])
    )
let subsequences = \(a : Type) -> \(xs : List a) ->
    (List/fold a xs (List (List a))
      (\(x : a) -> \(acc : List (List a)) -> 
        list-concat (List a)
          [
            ( list-map (List a) (List a) (\(xs : List a) -> list-cons a x xs ) acc ),
            acc
          ]
      )
      [ (list-empty a) ]
    )
let
  add-interval = \(root : Note) -> \(interval : Integer) -> { note = int-add root.note interval, channel = root.channel }
let 
  ByIntervalsSimpleT = {
    roots : List Note,
    intervals : List (List Note) -- every list can add a single note. that is: the result are subsequences of cartesian product of roots with intervals
  }
let
  -- interval notes end on a channel of the interval note
  -- so its possible to i.e. double a stream on two channels and send to two instruments
  -- todo: mapping of notes to notes
  -- todo: maybe treat root as just info and necessitate usage of 0 interval. this is a better ideas 
  by_intervals_simple = \(config : ByIntervalsSimpleT) ->
    let 
      intervals = concat-map (List Note) (List Note) 
      (\(chord : List Note) -> subsequences Note chord)
      (cross-product Note config.intervals)
    
    in concat-map Note (List Note)
      (\(root : Note) ->
        list-map (List Note) (List Note)
          (\(chord : List Note) -> list-concat Note [ [root], (list-map Note Note (\(n : Note) -> add-interval { note = root.note, channel = n.channel } n.note) chord)])
          intervals
      )
      config.roots
let
  -- a chord here gets defined by a root and a list of intervals played in relation to that root.
  -- when a chord is pressed - it will produce a played chord iff exactly one of the pressed notes defines a root
  -- and at least one defines an interval (without the 0 interval even the root won't be played).
  -- 
  -- So we provide two mappings: 
  --  - from pressed note to played root
  --  - from pressed notes to played intervals
  -- the second mapping is provided in a form of a list of mappings,
  -- from every mapping in the list a single key can be pressed (we reduce that way number of total chords in the configuration for midi-mapper).
  -- Any note as a key in the mapping should appear once: either in roots or in intervals. That's why the "roots" map can also contain interval notes.
  -- Also note that a single note can define multiple intervals (note a List in the type).
  ByIntervalsT = {
    -- roots
    roots : Mapping Note { root : Integer, intervals : List Note },
    -- a list of interval groups.
    -- a single note from an interval group CAN be triggered at one time
    intervals : List (Mapping Note { intervals : List Note }) -- every list can add a single note. that is: the result are subsequences of cartesian product of roots with intervals
  }
let
  by_intervals = \(config : ByIntervalsT) ->
    let PressedInterval = (Pair Note { intervals : List Note })
    let PressedRoot = (Pair Note { root : Integer, intervals : List Note })
    let intervals_from_roots = list-map PressedRoot PressedInterval
      (\(pair : PressedRoot) -> { key = pair.key ,  val = { intervals = pair.val.intervals } }) config.roots
    let interval_chords_list = list-filter (List PressedInterval)
      (\(xs : List PressedInterval) -> False == optional-null PressedInterval (List/head PressedInterval xs) )
      (
        concat-map (List PressedInterval) (List PressedInterval)
          -- a full chord can be pressed or really any subset of it, so we do `concatMap subsequences` and then filter the empty
          (\(chord : List PressedInterval) -> subsequences PressedInterval chord)
          -- cross product: every interval group gives a single possible key for press
          (cross-product PressedInterval
            -- combine the interval group from "config.roots" with the interval groups from "config.intervals"
            ( list-cons (List PressedInterval) intervals_from_roots config.intervals)
          )
      )
    let roots = list-map PressedRoot (Pair Note Integer)
      (\(pair : PressedRoot) -> { key = pair.key ,  val = pair.val.root }) config.roots

    in concat-map (Pair Note Integer) ChordPair
      (\(root : Pair Note Integer) ->
        -- add all chords starting at the given root
        list-map (List PressedInterval) ChordPair
          (\(xs : List PressedInterval) ->
            let IntervalChord = { intervals : List Note }
            let tmp = unzip-pairs Note IntervalChord xs
            let played_intervals = list-concat Note (list-map IntervalChord (List Note) (\(x : IntervalChord) -> x.intervals) tmp.val)
            let pressed_notes = tmp.key
            in {
              -- play when the note for root and all intervals are pressed
              key = chord (list-cons Note root.key pressed_notes), 
              -- interpret chord relative to root
              val = chord (
                list-map Note Note
                  (\(x : Note) ->
                    add-interval { note = root.val, channel = x.channel } x.note
                  )
                  played_intervals
                )
            }
          )
          interval_chords_list
      )
      roots
  let
    NoteIntervalPair = Pair Note { intervals : List Note }
  let 
    played_on_channels = \(channels : List Integer) -> \(xs : List NoteIntervalPair) ->
      list-map NoteIntervalPair NoteIntervalPair 
        (\(x : NoteIntervalPair) -> 
          { 
            key = x.key, 
            val = { intervals = concat-map Note Note (\(n: Note) -> list-map Integer Note (\(channel : Integer) -> { channel = channel, note = n.note }) channels) x.val.intervals }
          })
        xs
  let
    direct_mapped_intervals = \(xs : List Note) -> 
      list-map Note (Pair Note { intervals : List Note }) (\(x : Note) -> { key = x, val = { intervals = [ x ] } }) xs
  let
    direct_mapped_roots = \(xs : List Note) ->
      list-map Note (Pair Note { root : Integer, intervals : List Note })
        (\(x : Note) -> { key = x, val = { root = x.note, intervals = [ { note = +0, channel = x.channel } ] } })
        xs
  let
    -- list of notes send from mk2 in order reading row after row starting at bottom left
    novation-mk2 = concat-map Integer Note
      (\(tens: Integer) -> note-range { channel = +0, from = int-add tens +11, to = int-add tens +8 })
      [+10, +20, +30, +40, +50, +60, +70, +80]
  in {
  by_chord_type = by_chord_type,
  ChordTypes = ChordTypes,
  Chord = Chord,
  IntChordType = IntChordType,
  IntInt = IntInt,
  Note = Note,
  Pair = Pair,
  Configuration = Configuration,
  Mapping = Mapping,
  ByIntervalsT = ByIntervalsT,
  ChordPair = ChordPair,
  chord = chord,
  channel-zero = channel-zero,
  note-range = note-range,
  -- by_intervals_simple = by_intervals_simple,
  by_intervals = by_intervals,
  played_on_channels = played_on_channels,
  direct_mapped_intervals = direct_mapped_intervals,
  direct_mapped_roots = direct_mapped_roots,
  list-concat = list-concat,
  list-cons = list-cons,
  novation-mk2 = novation-mk2,
  list-unzip = list-unzip,
  list-map = list-map,
  list-filter = list-filter,
  list-empty = list-empty,
  range = range,
  int-add = int-add,
  zip-pairs = zip-pairs,
  unzip-pairs = unzip-pairs,
  list-mapWithIndex = list-mapWithIndex,
  list-index = list-index,
  optional-default = optional-default
}