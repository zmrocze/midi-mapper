{
  description = "midi-mapper";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    my-lib.url = "github:zmrocze/nix-lib";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, my-lib, crane, flake-utils, ... }:
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
          midi-mapper = craneLib.buildPackage (myLib.recursiveUpdateConcat individualCrateArgs {
            pname = "midi-mapper";
            src = fileSetForCrate ./exe;
            cargoExtraArgs = "--profile release --bin midi-mapper";
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
          midi-printer = craneLib.buildPackage (individualCrateArgs // {
            pname = "midi-printer";
            src = fileSetForCrate ./exe;
            cargoExtraArgs = "--profile release --bin midi-printer";
          });
          midi-mapper-wrapped = pkgs.writeShellApplication {
            name = "midi-mapper";
            runtimeInputs = [ pkgs.dhall-yaml midi-mapper ];
            text = ''
              midi-mapper "$@"
            '';
          };
          in {
            packages = {
              inherit midi-printer;
              midi-mapper = midi-mapper-wrapped;
            };
            apps = {
              midi-mapper = flake-utils.lib.mkApp {
                drv = midi-mapper-wrapped;
              };
              midi-printer = flake-utils.lib.mkApp {
                drv = midi-printer;
              };
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
