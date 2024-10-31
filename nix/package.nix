{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "vermeer-undervolt";
  version = "0.1.0";

  src = lib.cleanSource ./..;

  cargoHash = "sha256-+jOfMXgsLFnaa2BwiuJ4iCSrUfUrAryAFI57sXDxVHE=";

  meta = {
    description = "A command line utility to undervolt Ryzen Vermeer CPUs using Ryzen SMU";
    homepage = "https://github.com/nanoyaki/5800x3d-undervolt";
    license = lib.licenses.mit;
    platforms = lib.platforms.x86_64;
    mainProgram = "vermeer-undervolt";
  };
}
