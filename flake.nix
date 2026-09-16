{
  description = "A lightweight TUI dashboard for API health monitoring built with Rust and Ratatui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      version = cargoToml.package.version;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f (import nixpkgs { inherit system; }));

      # Release asset targets and sha256 hashes from GitHub Releases
      releaseAssets = {
        x86_64-linux = {
          target = "x86_64-unknown-linux-musl";
          hash = "sha256-S+JlhZD+F88HuJafucA2pTv71lH4gINHpG+Z7Y1ZuDQ=";
        };
        aarch64-linux = {
          target = "aarch64-unknown-linux-gnu";
          hash = "sha256-7EK2YtxdT63c70OlMTbIphVj2nx8BZwxiqXvfR4uUng=";
        };
        x86_64-darwin = {
          target = "x86_64-apple-darwin";
          hash = "sha256-FX6Aw5XuxasyB/TJ/v7cmMarMlMt593rvAZlTkkwwx0=";
        };
        aarch64-darwin = {
          target = "aarch64-apple-darwin";
          hash = "sha256-W4QlwQPlyYxLTPpcE7WlHV6TVjD3TWkJ9xV2N0sOFJk=";
        };
      };
    in
    {
      packages = forAllSystems (pkgs: rec {
        # Default: Instant pre-compiled binary from GitHub Releases
        default = statui;
        statui =
          let
            asset = releaseAssets.${pkgs.stdenv.hostPlatform.system};
          in
          pkgs.stdenv.mkDerivation {
            pname = "statui";
            inherit version;

            src = pkgs.fetchurl {
              url = "https://github.com/Mohamed-Badry/statui/releases/download/v${version}/statui-v${version}-${asset.target}.tar.gz";
              inherit (asset) hash;
            };

            sourceRoot = ".";

            nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux [
              pkgs.autoPatchelfHook
            ];

            installPhase = ''
              runHook preInstall
              install -Dm755 statui $out/bin/statui
              runHook postInstall
            '';

            meta = with pkgs.lib; {
              description = cargoToml.package.description;
              homepage = cargoToml.package.repository;
              license = licenses.mit;
              mainProgram = "statui";
            };
          };

        # Source build using nixpkgs rustPlatform
        source = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          inherit version;

          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./src
            ];
          };

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          buildInputs = pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin (
            with pkgs.darwin.apple_sdk.frameworks;
            [
              CoreFoundation
              Security
              SystemConfiguration
            ]
          );

          meta = default.meta;
        };
      });

      apps = forAllSystems (pkgs: rec {
        default = statui;
        statui = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.default}/bin/statui";
        };
        source = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.source}/bin/statui";
        };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.stdenv.hostPlatform.system}.source ];
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
          ];

          RUST_BACKTRACE = "1";
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);

      overlays.default = final: prev: {
        statui = self.packages.${final.stdenv.hostPlatform.system}.default;
        statui-source = self.packages.${final.stdenv.hostPlatform.system}.source;
      };
    };
}
