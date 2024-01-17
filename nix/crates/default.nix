{ inputs, ... }: {
  perSystem = { pkgs, config, lib, ... }: {
    nci = {
      projects.ltl-actus = {
        path = inputs.self;
        # export = true;
        # drvConfig.mkDerivation.buildInputs = with pkgs; [
        #   rustup
        #   zulu
        # ]; # for prusti
        # # Even tho vscode can't pick it up ephemerally and it needs user installed rustup and java.
      };
      crates = {
        ltl = import ./ltl.nix { inherit pkgs config lib; };
        actus = import ./actus.nix { inherit pkgs config lib; };
        ltl_actus_cli = import ./cli.nix { inherit pkgs config; };
      };
    };
    apps.default.program =
      "${config.nci.outputs.ltl_actus_cli.packages.release}/bin/ltl_actus_cli";
    packages.default = config.nci.outputs.ltl_actus_cli.packages.release;
  };
}
