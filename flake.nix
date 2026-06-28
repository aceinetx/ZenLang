{
  description = "zenlang";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            inputs.fenix.overlays.default
          ];
        };
        inherit (pkgs) lib stdenv;

        rust-toolchain = pkgs.fenix.stable;
        rust-platform = pkgs.makeRustPlatform {
          inherit (rust-toolchain) rustc cargo;
        };
      in
      rec {
        defaultPackage = rust-platform.buildRustPackage (finalAttrs: {
          name = "graphfix";

          src = ./.;

          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];

          runtimeInputs = lib.optionals stdenv.hostPlatform.isLinux (
            with pkgs;
            [
              wayland
              vulkan-loader
              libxkbcommon
            ]
          );

          postFixup = lib.optional (finalAttrs.runtimeInputs != [ ]) ''
            for executable in $(find $out/bin -type f -executable); do
              wrapProgram $executable \
                --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath finalAttrs.runtimeInputs}
            done
          '';

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        });

        devShell = pkgs.mkShell {
          name = "${defaultPackage.name}-dev";

          inputsFrom = [ defaultPackage ];

          packages = with pkgs; [
            rust-analyzer
            clippy
            rustfmt

            gdb

            cargo-feature
            cargo-outdated
            cargo-audit
          ];

          LD_LIBRARY_PATH = lib.makeLibraryPath defaultPackage.runtimeInputs;

          RUST_LOG = "debug";
          RUST_SRC_PATH = "${rust-toolchain.rust-src}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = 1;
        };
      }
    );
}
