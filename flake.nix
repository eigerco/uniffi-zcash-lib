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
            ./nix/rust.nix
            {
              languages = {
                elixir.enable = true;
                erlang.enable = true;
                ruby.enable = true;
                # Nixified Swift is currently broken on Darwin, so instead you
                # should use system one
                swift.enable = !pkgs.stdenv.isDarwin;
                kotlin.enable = true;
                python.enable = true;
              };
            }
          ];
        };
      });
}
