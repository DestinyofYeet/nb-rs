{ rustPlatform, lib, self, pkgs, ... }:
let
  version = toString
    (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");

in rustPlatform.buildRustPackage {
  pname = "nb-rs";
  version = "1.0";

  nativeBuildInputs = with pkgs; [ git ];

  src = ./.;

  cargoHash = "sha256-+nuyXr4XncXG9hYzs2rgg5pO9IKNMtqAj9N1bHto+6o=";

  GIT_REV = version;

  meta = with lib; {
    description = "nb-rs";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
