{ ... }: {
  perSystem = { config, pkgs, ... }: {
    devShells = {
      bootstrap = with pkgs;
        mkShell { buildInputs = [ rustup libressl pkg-config ]; };
      default = config.nci.outputs.ltl-actus.devShell;
    };
  };
}
