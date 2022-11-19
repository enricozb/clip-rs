{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "clip-rs";
  version = "0.0.2";

  src = pkgs.fetchFromGitHub {
    owner = "enricozb";
    repo = "clip-rs";
    rev = "v${version}";
    sha256 = "sha256-QLH6OJsU+9gf+NT2AOP7a4OxHyn9w4Wh+VQAjZNlsXo=";
  };

  cargoLock = { lockFile = ./Cargo.lock; };

  postInstall = ''
    mv $out/bin/clip-rs $out/bin/clip
  '';
}
