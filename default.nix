{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "clip-rs";
  version = "0.0.1";

  src = pkgs.fetchFromGitHub {
    owner = "enricozb";
    repo = "clip-rs";
    rev = "v${version}";
    sha256 = "sha256-rWdhH5783rLMMDMzfOcpiD3+fpGoExgfDrum3rR6BCk=";
  };

  cargoLock = { lockFile = ./Cargo.lock; };

  postInstall = ''
    mv $out/bin/clip-rs $out/bin/clip
  '';
}
