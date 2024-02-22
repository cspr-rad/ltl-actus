{ inputs, ... }: {
  perSystem = { pkgs, config, ... }: {
    packages.model = pkgs.stdenv.mkDerivation {
      name = "ltl-model-spec";
      src = "${inputs.self}/ltl-model";
      buildInputs = [ pkgs.lean4 ];
      buildPhase = "lake build";
      installPhase = ''
        mkdir -p $out
        cp -r .lake/* $out
      '';
    };
  };
}
