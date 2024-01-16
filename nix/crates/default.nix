{ inputs, ... }: {
  perSystem = { pkgs, config, ... }: {
    nci = {
      projects.ltl-actus = {
        path = inputs.self;
        export = true;
      };
      crates = {
        ltl = import ./ltl.nix { inherit pkgs config; };
        actus = import ./actus.nix { inherit pkgs config; };
        ltl_actus_cli = import ./cli.nix { inherit pkgs config; };
      };
    };
    apps.default.program = "${config.nci.outputs.ltl_actus_cli.packages.release}/bin/ltl_actus_cli";
    packages.default = config.nci.outputs.ltl_actus_cli.packages.release;
  };
}
