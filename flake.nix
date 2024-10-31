{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ flake-parts, ... }:

    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./nix/nixosModule.nix
      ];

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

          packages.default = pkgs.callPackage ./nix/package.nix { };
        };

      systems = [ "x86_64-linux" ];
    };
}
