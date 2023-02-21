{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    devenv.url = "github:cachix/devenv";
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell
          {
            packages = with pkgs; let
              beamPackages = beam.packages.erlangR25;
              rustPackages = fenix.packages.${pkgs.system}.stable;
            in
            [
              # Elixir
              beamPackages.erlang
              beamPackages.elixir_1_14
              beamPackages.elixir_ls

              # Rust
              libiconv
              uniffi-bindgen
              rustPackages.rustc
              rustPackages.cargo
              rustPackages.rustfmt
              rustPackages.rust-src
              rustPackages.clippy
              rustPackages.rust-analyzer

              # Ruby
              ruby_3_1

              # Python
              python311

              # Kotlin
              kotlin
              gradle
            ]
            ++ lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.Foundation
            ];
          };
      });
}
