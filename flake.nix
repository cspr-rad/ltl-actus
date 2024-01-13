{
  description = "Execute ACTUS traces in linear temporal logic";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-pandoc = {
      url = "github:serokell/nix-pandoc";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    beamer-theme-serokell = {
      url = "github:serokell/beamer-theme-serokell";
      flake = false;
    };
    parts.url = "github:hercules-ci/flake-parts";
    fmt = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ self, nixpkgs, nci, nix-pandoc, beamer-theme-serokell
    , parts, fmt }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-darwin" "x86_64-darwin" ];
      imports = [
        nci.flakeModule
        ./nix/crates
        ./nix/shells.nix
        ./nix/comms
        fmt.flakeModule
        ./nix/format.nix
      ];
    };
}
