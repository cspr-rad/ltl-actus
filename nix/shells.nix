{ ... }: {
  perSystem = { config, pkgs, ... }: {
    devShells = let greeting = "ACTUS in linear temporal logic";
    in {
      rs = config.nci.outputs.ltl-actus.devShell;
      c = with pkgs;
        mkShell {
          name = "c-programming-ltlactus";
          shellHook = "echo ${greeting}";
          buildInputs = [ cmake gnumake pkg-config glib irony-server ];
        };
      lean = with pkgs;
        mkShell {
          name = "ltlactus-spec";
          shellHook = "echo ${greeting}";
          buildInputs = [ lean4 ];
        };
    };
  };
}
