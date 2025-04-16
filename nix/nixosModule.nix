{ self }:

rec {
  default = vermeer-undervolt;
  vermeer-undervolt =
    {
      pkgs,
      lib,
      config,
      ...
    }:

    let
      inherit (lib)
        types
        mkIf
        mkOption
        mkEnableOption
        ;

      cfg = config.services.vermeer-undervolt;
      exec = lib.getExe self.packages.${pkgs.stdenv.hostPlatform.system}.default;
    in

    {
      options.services.vermeer-undervolt = {
        enable = mkEnableOption "undervolting options for AMD Ryzen CPUs";

        cores = mkOption {
          type = types.int;
          default = 0;
          example = 8;
          description = ''
            The amount of cores to apply the undervolt to
          '';
        };

        milivolts = mkOption {
          type = types.int;
          default = 0;
          example = 30;
          description = ''
            The milivoltage to reduce on the cores
          '';
        };
      };

      config = mkIf cfg.enable {
        hardware.cpu.amd.ryzen-smu.enable = true;

        systemd.services.vermeer-undervolt = {
          enable = true;
          description = "Undervolt for AMD Ryzen CPUs";

          wantedBy = [ "multi-user.target" ];

          serviceConfig = {
            ExecStart = "${exec} ${toString cfg.cores} -${toString cfg.milivolts}";
            User = "root";
            Group = "wheel";
          };
        };
      };
    };
}
