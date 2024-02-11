{ self, pkgs }:
pkgs.stdenv.mkDerivation {
  name = "ltl-actus-jan2024-article";
  buildInputs = [ pkgs.pandoc ];
  src = "${self}/comms/january-2024-actus-workshop";
  buildPhase = ''
    pandoc -t docx -f org $src/medium-article.org -o article.docx
  '';
  installPhase = ''
    mkdir -p $out
    cp article.docx $out/article.docx
  '';
}
