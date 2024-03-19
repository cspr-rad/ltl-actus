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
  name = "ltl-actus-fv-reactive-system_conference-talk";
  src = "${self}/comms/may-2024-actus-dc";
  inherit texlive-combined;
  HOME = "/build";
  extraTexInputs = [ beamer-theme-serokell ];
  extraBuildInputs = [ which pygments ];
}
