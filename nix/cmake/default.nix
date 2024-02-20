{ inputs, ... }: {
  perSystem = { pkgs, config, ... }: {
    packages.cmake = pkgs.stdenv.mkDerivation {
      name = "ltlactus-cmake";
      buildInputs = with pkgs; [ cmake gnumake pkg-config glib ];
      src = "${inputs.self}/c";
      buildPhase = ''
        cmake .
        make
      '';
    };
  };
}
