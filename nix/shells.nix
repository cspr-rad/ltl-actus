{ ... }: {
  perSystem = { config, pkgs, ... }: {
    devShells = let greeting = "Welcome to ACTUS in linear temporal logic";
    in {
      bootstrap = with pkgs;
        mkShell {
          shellHook = "echo ${greeting}";
          buildInputs = [ rustup libressl pkg-config ];
        };
      rs = config.nci.outputs.ltl-actus.devShell;
      c = with pkgs;
        mkShell {
          name = "c-programming-ltlactus";
          shellHook = "echo ${greeting}";
          buildInputs = [ cmake gnumake pkg-config glib ];
        };
    };
  };
}
