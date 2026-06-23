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

  cargoHash = "sha256-no192IPH6dN1qSxG91oCkTqK1J4vbiXJLluuFZmNLTY=";

  GIT_REV = version;

  meta = with lib; {
    description = "nb-rs";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
