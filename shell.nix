{ pkgs ? import <nixpkgs> {} }:

let 
  nix_lib = builtins.getFlake "git+https://github.com/zmrocze/nix-lib";
in
pkgs.mkShell {
  packages = with pkgs ; [ 
  	# pkgs.alsa-plugins
  	alsa-lib
  	pkg-config
  	dhall-yaml
  	dhall-lsp-server
  	dhall
  	# nix_lib.devShells.${builtins.currentSystem}.default
  ];
}
