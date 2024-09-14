
# midi mapper

Virtual MIDI device mapping the incoming chords to chords it outputs, as defined in the mapping in config.
Comes with a simple dhall library for defining the configs, see examples in [./exe/src/midi_mapper/resources/](./exe/src/midi_mapper/resources/).

```bash
nix run .#midi_mapper -- --help
```

for configs see the default in `exe/midi_mapper/resources`. There's dhall and yaml versions, dhall is source to yaml with `dhall-to-yaml-ng`.

# midi printer

Prints incoming midi notes and forwards.

```bash
nix run .#midi_printer -- --help
```
