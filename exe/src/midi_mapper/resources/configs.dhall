
-- let 
--   lib = https://raw.githubusercontent.com/zmrocze/midi-mapper/develop/exe/src/midi_mapper/dhall/lib.dhall
let
  debug = ~/code/rust/midi-mapper/exe/src/midi_mapper/resources/debug.dhall
let
  minifreak = ~/code/rust/midi-mapper/exe/src/midi_mapper/resources/./minifreak.dhall
let
  multi_channel = ~/code/rust/midi-mapper/exe/src/midi_mapper/resources/multi_channel.dhall
let
  simple_config = ~/code/rust/midi-mapper/exe/src/midi_mapper/resources/simple_config.dhall
let
  mk2_3by4 = ~/code/rust/midi-mapper/exe/src/midi_mapper/resources/mk2_3by4.dhall
let 
  default_config = ~/code/rust/midi-mapper/exe/src/midi_mapper/resources/default_config.dhall
in
  debug /\ minifreak /\ multi_channel /\ simple_config
