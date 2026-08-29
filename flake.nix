{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs =
    { self, nixpkgs, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        toml = fromTOML (builtins.readFile ./Cargo.toml);

        formattingConfig =
          { ... }:
          {
            projectRootFile = "flake.nix";
            programs = {
              nixfmt.enable = true;
              rustfmt = {
                enable = true;
                edition = toml.package.edition;
              };

              sql-formatter = {
                enable = true;
                dialect = "sqlite";
              };

              mdformat.enable = true;
              jsonfmt.enable = true;

              # js / ts / css / scss
              prettier.enable = true;

              leptosfmt.enable = true;

              toml-sort.enable = true;

              ruff-format.enable = true;
            };
          };

        treeFmtEval = inputs.treefmt-nix.lib.evalModule pkgs formattingConfig;

      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            clippy
            # openssl
            # pkg-config
            rust-analyzer
            rustfmt # formatter
          ];

          # uncomment this is you get some kind of ssl error, usually on anything networking related using reqwest
          # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };

        packages.default = pkgs.callPackage ./pkg.nix { inherit self toml; };

        formatter = treeFmtEval.config.build.wrapper;

        checks = {
          formatting = treeFmtEval.config.build.check self;
        };
      }
    );
}
