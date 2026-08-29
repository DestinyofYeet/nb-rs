{
  rustPlatform,
  lib,
  self,
  pkgs,
  toml,
  ...
}:
let
  version = toString (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");

in
rustPlatform.buildRustPackage {
  pname = toml.package.name;
  version = toml.package.version;

  nativeBuildInputs = with pkgs; [ git ];

  src = ./.;

  cargoHash = "sha256-KbIJIKU94X3CbAb1goUOCSh6+Mia/eXyXxF0nHwfejg=";

  GIT_REV = version;

  meta = with lib; {
    description = "nb-rs";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
