{ pkgs ? import <nixpkgs> {} }:

let 
  nix_lib = builtins.getFlake "git+https://github.com/zmrocze/nix-lib";
in
pkgs.mkShell {
  packages = [ 
  	# pkgs.alsa-plugins
  	pkgs.alsa-lib
  	pkgs.pkg-config
  	# nix_lib.devShells.${builtins.currentSystem}.default
  ];
}
