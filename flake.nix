# 🐻‍❄️💐 ume: Easy, self-hostable, and flexible image host made in Rust
# Copyright 2021-2024 Noel Towa <cutie@floofy.dev>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
{
  description = "🐻‍❄️💐 Easy, self-hostable, and flexible image host made in Rust";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;

        overlays = [(import rust-overlay)];
        config.allowUnfree = true;
      };

      package = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      stdenv =
        if pkgs.stdenv.isLinux
        then pkgs.stdenv
        else pkgs.clangStdenv;

      rustflags =
        if pkgs.stdenv.isLinux
        then ''-C link-arg=-fuse-ld=mold -C target-cpu=native $RUSTFLAGS''
        else ''$RUSTFLAGS'';
    in rec {
      packages = {
        ume = pkgs.rustPlatform.buildRustPackage {
          nativeBuildInputs = with pkgs; [pkg-config];
          buildInputs = with pkgs; [openssl];
          cargoSha256 = pkgs.lib.fakeSha256;
          version = "${package.version}";
          name = "ume";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "noelware-config-0.1.0" = pkgs.lib.fakeSha256;
              "noelware-config-derive-0.1.0" = pkgs.lib.fakeSha256;
              "noelware-log-0.1.0" = pkgs.lib.fakeSha256;
              "noelware-remi-0.1.0" = pkgs.lib.fakeSha256;
              "noelware-serde-0.1.0" = pkgs.lib.fakeSha256;
            };
          };
          meta = with pkgs.lib; {
            description = "Easy, self-hostable, and flexible image host made in Rust";
            homepage = "https://github.com/auguwu/ume";
            license = with licenses; [asl20];
            maintainers = with maintainers; [auguwu];
            mainProgram = "ume";
          };
        };

        default = packages.ume;
      };

      devShells.default = pkgs.mkShell {
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [openssl]);
        nativeBuildInputs = with pkgs;
          [pkg-config]
          ++ (lib.optional stdenv.isLinux [mold lldb gdb])
          ++ (lib.optional stdenv.isDarwin [darwin.apple_sdk.frameworks.CoreFoundation]);

        buildInputs = with pkgs; [
          cargo-expand
          openssl
          glibc
          rust
          git
        ];
      };
    });
}
