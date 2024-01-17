{ pkgs, config, lib }:

{
  targets = {
    "wasm32-unknown-unknown" = {
      default = false;
      drvConfig.mkDerivation = {
        # add trunk and other dependencies
        nativeBuildInputs = with pkgs; [
          trunk
          nodePackages.sass
          wasm-bindgen-cli
        ];
        # override build phase to build with trunk instead
        buildPhase = lib.mkForce ''
          export TRUNK_TOOLS_SASS="${pkgs.nodePackages.sass.version}"
          export TRUNK_TOOLS_WASM_BINDGEN="${pkgs.wasm-bindgen-cli.version}"
          echo sass is version $TRUNK_TOOLS_SASS
          echo wasm bindgen is version $TRUNK_TOOLS_WASM_BINDGEN
          HOME=$TMPDIR \
            trunk -v build \
            --dist $out \
            --release \
            ''${cargoBuildFlags:-}
        '';
        # disable install phase because trunk will directly output to $out
        dontInstall = true;
      };
    };
    "x86_64-unknown-linux-gnu" = { default = true; };
  };
}
