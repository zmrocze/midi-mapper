{
  description = "midi_mapper";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    my-lib.url = "github:zmrocze/nix-lib";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    midi-mapper-og = "github:zmrocze/midi-mapper/41e9610";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, my-lib, crane, flake-utils, midi-mapper-og, ... }:
    let
      myLib = my-lib.lib;
    in
    flake-parts.lib.mkFlake { inherit inputs; }
      {      
        imports = [
          inputs.my-lib.flakeModules.pkgs
        ];
        pkgsConfig = {
          overlays = [
            inputs.my-lib.overlays.default
          ];
          # systems = [ "x86_64-linux" ];
        };
        systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
        perSystem = { pkgs, lib, system, ... }: let 
          craneLib = crane.mkLib pkgs;
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          commonArgs = {
            inherit src;
            strictDeps = true;
            cargoExtraArgs = "--profile release";
            nativeBuildInputs = with pkgs; [
              # alsa
              # alsa-lib
  	          pkg-config
            ];
            # todo: check order if changes sth inside single *inputs, gh issue add this
            buildInputs = with pkgs; [
              alsa-lib
            ];
            PKG_CONFIG_PATH = "${pkgs.alsa-lib}/lib/pkgconfig";
          };
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          individualCrateArgs = commonArgs // {
            inherit cargoArtifacts;
            inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
            doCheck = false;
          };

          fileSetForCrate = crate: lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./utils
              ./lib
              crate
            ];
          };
          # Build the top-level crates of the workspace as individual derivations.
          # This allows consumers to only depend on (and build) only what they need.
          # Though it is possible to build the entire workspace as a single derivation,
          # so this is left up to you on how to organize things
          midi_mapper = craneLib.buildPackage (myLib.recursiveUpdateConcat individualCrateArgs {
            pname = "midi_mapper";
            src = fileSetForCrate ./exe;
            cargoExtraArgs = "--profile release --bin midi_mapper";
            runtimeInputs = with pkgs; [
              dhall-yaml
            ];
            nativeBuildInputs = with pkgs; [
              dhall-yaml
            ];
            buildInputs = with pkgs; [
              dhall-yaml
            ];
          });
          midi_printer = craneLib.buildPackage (individualCrateArgs // {
            pname = "midi_printer";
            src = fileSetForCrate ./exe;
            cargoExtraArgs = "--profile release --bin midi_printer";
          });
          midi_mapper-wrapped = pkgs.writeShellApplication {
            name = "midi_mapper";
            runtimeInputs = [ pkgs.dhall-yaml midi_mapper ];
            text = ''
              midi_mapper "$@"
            '';
          };
          in {
            packages = {
              inherit midi_printer;
              midi_mapper = midi_mapper-wrapped;
              og-midi-mapper = midi-mapper-og.packages.${system}.midi_mapper;
            };
            apps = {
              midi_mapper = flake-utils.lib.mkApp {
                drv = midi_mapper-wrapped;
              };
              midi_printer = flake-utils.lib.mkApp {
                drv = midi_printer;
              };
              og-midi-mapper = midi-mapper-og.apps.${system}.midi_mapper;
            };

            devShells.default = craneLib.devShell {
              # Inherit inputs from checks.
              checks = self.checks.${system};

              # Additional dev-shell environment variables can be set directly
              # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

              # Extra inputs can be added here; cargo and rustc are provided by default.
              packages = [
              ];
            };
          };
      };
}
