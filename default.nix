{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "clip-rs";
  version = "0.0.4";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };

  postInstall = ''
    mv $out/bin/clip-rs $out/bin/clip
  '';
}
