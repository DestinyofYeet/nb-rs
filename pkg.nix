{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "nb-rs";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-Dxoi4Hw1i5rioKkQwF2Vw2VAIel8L3OokmPu37wXZu0=";

  meta = with lib; {
    description = "nb-rs";
    license = licenses.agpl3Only;
    platforms = platforms.all;
  };
}
