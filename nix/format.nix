{ inputs, ... }:
{
  perSystem = { pkgs, config, ... }:
    {
      treefmt.config = {
        projectRootFile = "flake.nix";
        programs = {
          rustfmt.enable = true;
          nixpkgs-fmt.enable = true;
        };
      };
    };
}
