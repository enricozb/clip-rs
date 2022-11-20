{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "clip-rs";
  version = "0.0.3";

  src = pkgs.fetchFromGitHub {
    owner = "enricozb";
    repo = "clip-rs";
    rev = "v${version}";
    sha256 = "sha256-8Vmw8HYeMGSSyNSjiuRphZozNkCjgev3pzZvDwEs7f4=";
  };

  cargoLock = { lockFile = ./Cargo.lock; };

  postInstall = ''
    mv $out/bin/clip-rs $out/bin/clip
  '';
}
