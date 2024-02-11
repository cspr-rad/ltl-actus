{ inputs, ... }: {
  perSystem = { config, pkgs, ... }: {
    packages = {
      talk = (pkgs.extend inputs.nix-pandoc.overlay).callPackage
        ./january-2024-actus-workshop/talk.nix {
          pygments = pkgs.python311Packages.pygments;
          inherit (inputs) beamer-theme-serokell self;
        };
      article = import ./january-2024-actus-workshop/article.nix {
        inherit (inputs) self;
        inherit pkgs;
      };
    };
  };
}
