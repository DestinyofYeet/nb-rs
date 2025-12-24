{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "nb-rs";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-+nuyXr4XncXG9hYzs2rgg5pO9IKNMtqAj9N1bHto+6o=";

  meta = with lib; {
    description = "nb-rs";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
