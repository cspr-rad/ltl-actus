{ inputs, ... }: {
  perSystem = { pkgs, config, ... }:
    {
      nci = {
        projects.ltl-actus = {
          path = inputs.self;
          export = true;
        };
        crates = {
          ltl = import ./ltl.nix { inherit pkgs config; };
          actus = import ./actus.nix { inherit pkgs config; };
          cli = import ./cli.nix { inherit pkgs config; };
        };
      };
    };
}
