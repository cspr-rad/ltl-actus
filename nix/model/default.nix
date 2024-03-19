{ inputs, ... }: {
  perSystem = { pkgs, config, ... }: {
    packages.model = pkgs.stdenv.mkDerivation {
      name = "ltl-actus-spec";
      src = "${inputs.self}/lean";
      buildInputs = [ pkgs.lean4 ];
      buildPhase = "lake build";
      installPhase = ''
        mkdir -p $out
        cp -r .lake/* $out
      '';
    };
  };
}
