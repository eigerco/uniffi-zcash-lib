{ pkgs, inputs, ... }:
let
  fenix = inputs.fenix;
  rustPackages = fenix.packages.${pkgs.system}.stable;
in {
  packages = with pkgs; [
    libiconv
    uniffi-bindgen
    rustPackages.rustc
    rustPackages.cargo
    rustPackages.rustfmt
    rustPackages.rust-src
    rustPackages.clippy
    rustPackages.rust-analyzer
  ]
    ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.Foundation
  ];
}
