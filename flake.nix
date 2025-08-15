{
  description = "Oxide + Sandbox dev shell with Wayland support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            pkg-config
            wayland            # main Wayland client library
            wayland-protocols  # extra Wayland protocols for some apps
            libxkbcommon       # needed for keyboard input
            xorg.libX11        # if you also want X11 fallback
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];

          # This ensures pkg-config can find Wayland headers/libs
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXi
              pkgs.xorg.libXrandr
            ]}:$LD_LIBRARY_PATH"
            echo "Wayland dev shell ready."
          '';
        };
      }
    );
}
