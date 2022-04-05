{
  inputs = {

    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs =
    { self
    , nixpkgs
    , fenix
    , gitignore
    }:
    let
      inherit (gitignore.lib) gitignoreSource;

      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system:
        import nixpkgs {
          inherit system;
          overlays = [ self.overlay ];
        });

      chanspec = {
        date = "2022-02-06";
        channel = "nightly";
        sha256 = "oKkTWopCDx4tphzTtRn+zDDtvmIZrL/H44tV2ruSfDw="; # set zeros after modifying channel or date
      };
      rustChannel = p: (fenix.overlay p p).fenix.toolchainOf chanspec;

    in
    {

      overlay = final: prev: {

        eva = with final;
          let
            pname = "eva";
            packageMeta = (lib.importTOML ./Cargo.toml).package;
            rustPlatform = makeRustPlatform {
              inherit (rustChannel final) cargo rustc;
            };
          in
          rustPlatform.buildRustPackage {
            inherit pname;
            inherit (packageMeta) version;

            src = gitignoreSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

      };

      packages = forAllSystems (system: {
        inherit (nixpkgsFor."${system}") eva;
      });

      defaultPackage =
        forAllSystems (system: self.packages."${system}".eva);

      devShell = forAllSystems (system:
        let
          pkgs = nixpkgsFor."${system}";
          toolchain = (rustChannel pkgs).withComponents [
            "rustc"
            "cargo"
            "rust-std"
            "rustfmt"
            "clippy"
            "rust-src"
          ];
          inherit (fenix.packages."${system}") rust-analyzer;
        in
        pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.bacon
            pkgs.cargo-insta
            rust-analyzer
            toolchain
          ];
          RUST_LOG = "info";
          RUST_BACKTRACE = 1;
        });

    };
}
