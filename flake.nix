{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          rust-analyzer
          pkg-config
          openssl
          cargo
          clippy
        ];
      };

      packages.x86_64-linux.default = pkgs.callPackage ./nix/package.nix { };
      packages.x86_64-linux.vermeer-undervolt = pkgs.callPackage ./nix/package.nix { };

      nixosModules = import ./nix/nixosModule.nix { inherit self; };
    };
}
