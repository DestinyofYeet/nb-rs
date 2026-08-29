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

  cargoHash = "sha256-hIeNS4IKdRpZhHj6OGEcZIo2wE35m23lCbxU10yc/qw=";

  GIT_REV = version;

  meta = with lib; {
    description = "nb-rs";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
