{ ... }: {
  perSystem = { config, pkgs, ... }: {
    devShells = let greeting = "Welcome to ACTUS in linear temporal logic";
    in {
      rs = config.nci.outputs.ltl-actus.devShell;
      c = with pkgs;
        mkShell {
          name = "c-programming-ltlactus";
          shellHook = "echo ${greeting}";
          buildInputs = [ cmake gnumake pkg-config glib irony-server ];
        };
      model = with pkgs;
        mkShell {
          name = "model-spec-ltlactus";
          shellHook = "echo ${greeting}";
          buildInputs = [ lean4 ];
        };
    };
  };
}
