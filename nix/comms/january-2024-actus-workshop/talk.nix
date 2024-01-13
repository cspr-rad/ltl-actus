{ self, beamer-theme-serokell, runCommand, mkDoc, texlive, pandoc, fontconfig
, pygments, which }:
let
  texlive-packages = {
    inherit (texlive)
      scheme-small noto mweights cm-super cmbright fontaxes beamer minted
      fvextra catchfile xstring framed;
  };

  texlive-combined = texlive.combine texlive-packages;

in mkDoc {
  name = "ltl-actus-towards-verification_workshop-talk";
  src = "${self}/comms/january-2024-actus-workshop";
  inherit texlive-combined;
  HOME = "/build";
  extraTexInputs = [ beamer-theme-serokell ];
  extraBuildInputs = [ which pygments ];
}
