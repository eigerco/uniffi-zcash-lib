{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    devenv.url = "github:cachix/devenv";
  };

  outputs = { nixpkgs, flake-utils, devenv, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = devenv.lib.mkShell {
          inherit pkgs inputs;

          modules = [
            {
              languages = {
                elixir.enable = true;
                erlang.enable = true;
                rust.enable = true;
                ruby.enable = true;
                swift.enable = !pkgs.stdenv.isDarwin;
                kotlin.enable = true;
                python.enable = true;
              };

              packages = with pkgs; [ uniffi-bindgen ]
                ++ lib.optionals stdenv.isDarwin [
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.Foundation
              ];
            }
          ];
        };
      });
}
