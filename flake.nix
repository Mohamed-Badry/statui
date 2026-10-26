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
          hash = "sha256-CPXESfFDZ7+XXiOljO8SPWKcTTCQ6tQ9ckmA962QCIw=";
        };
        aarch64-linux = {
          target = "aarch64-unknown-linux-gnu";
          hash = "sha256-x29MsJpLxJi+/sZKYE3JxRj28peCgvGSC2/P+/5sqms=";
        };
        x86_64-darwin = {
          target = "x86_64-apple-darwin";
          hash = "sha256-tUgpMeO+ubw976Z831TYXvgoEZrSKerpl9WlailbAMs=";
        };
        aarch64-darwin = {
          target = "aarch64-apple-darwin";
          hash = "sha256-/jQtVioLzwcNo+ybQCwzTTK++6apSyWhM5BwMW9U5Cw=";
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
          buildInputs = pkgs.lib.optionals pkgs.stdenv.hostPlatform.isDarwin (
            with pkgs.darwin.apple_sdk.frameworks;
            [
              CoreFoundation
              Security
              SystemConfiguration
            ]
          );
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
            cargo-audit
            cargo-nextest
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
