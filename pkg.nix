{
  rustPlatform,
  lib,
  self,
  pkgs,
  ...
}:
let
  version = toString (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");

in
rustPlatform.buildRustPackage {
  pname = "nb-rs";
  version = "1.0";

  nativeBuildInputs = with pkgs; [ git ];

  src = ./.;

  cargoHash = "sha256-lYQEK8FBLO57w1Qys8aAwx2/ie5sfYvXiSG0q24470A=";

  GIT_REV = version;

  meta = with lib; {
    description = "nb-rs";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
