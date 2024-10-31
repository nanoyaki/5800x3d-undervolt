{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:

    flake-parts.lib.mkFlake { inherit inputs; } {
      perSystem =
        { pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              rustc
              rust-analyzer
              pkg-config
              openssl
              cargo
              clippy
            ];
          };
        };

      systems = [ "x86_64-linux" ];
    };
}
