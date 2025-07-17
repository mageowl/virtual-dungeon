{
  description = "flake for macroquad development.";

  inputs.nixpkgs.url = "nixpkgs/nixpkgs-unstable";

  outputs = inputs: let
    system = "x86_64-linux";
    pkgs = inputs.nixpkgs.legacyPackages.${system};
  in {
    devShell.${system} = pkgs.mkShell {
      name = "macroquad-shell";
      buildInputs = with pkgs; [
        rustc
        cargo

        libGL
        xorg.libX11
        xorg.libXi
        libxkbcommon

        jdk24
      ];

      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
          with pkgs;
            lib.makeLibraryPath [libGL xorg.libX11 xorg.libXi libxkbcommon]
        }"
      '';
    };
  };
}
