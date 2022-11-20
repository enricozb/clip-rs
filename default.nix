{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "clip-rs";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };

  postInstall = ''
    mv $out/bin/clip-rs $out/bin/clip
  '';
}
