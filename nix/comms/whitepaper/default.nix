{ self, pkgs }:
pkgs.stdenv.mkDerivation {
  name = "ltl-actus-whitepaper";
  buildInputs = [ pkgs.pandoc pkgs.texliveSmall ];
  src = "${self}/comms/whitepaper/";
  buildPhase = ''
    pandoc -t latex -f org $src/source.org -o whitepaper.pdf
  '';
  installPhase = ''
    mkdir -p $out
    cp whitepaper.pdf $out/whitepaper.pdf
  '';
}
