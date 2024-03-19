{ inputs, ... }: {
  perSystem = { config, pkgs, ... }: {
    packages = {
      zurich-talk = (pkgs.extend inputs.nix-pandoc.overlay).callPackage
        ./january-2024-actus-workshop/talk.nix {
          pygments = pkgs.python311Packages.pygments;
          inherit (inputs) beamer-theme-serokell self;
        };
      medium-article-1 = import ./january-2024-actus-workshop/article.nix {
        inherit (inputs) self;
        inherit pkgs;
      };
      dc-talk = (pkgs.extend inputs.nix-pandoc.overlay).callPackage
        ./may-2024-actus-dc/talk.nix {
          pygments = pkgs.python311Packages.pygments;
          inherit (inputs) beamer-theme-serokell self;
        };
    };
  };
}
