{ lib, fetchFromGitHub, rustPlatform, installShellFiles }:

rustPlatform.buildRustPackage rec {
  pname = "clip-rs";
  version = "0.0.1";

  src = fetchFromGitHub {
    owner = "enricozb";
    repo = "clip-rs";
    rev = "v${version}";
    sha256 = "sha256-8QQHLw+isXtr1FDQr4aiUhvOjJUPbaxFGDwukiWBG9g=";
  };

  cargoSha256 = "sha256-1Fh47Pr+7lIdT++huziKgMJxvsZElTTwu11c7/wjyHE=";

  postInstall = ''
    ls
  '';
}
