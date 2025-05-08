{
  pkgs ? import <nixpkgs> {}
}:

pkgs.mkShell rec {
  packages = with pkgs; [
    heaptrack
  ];
  buildInputs = with pkgs; [
    expat
    fontconfig
    freetype
    freetype.dev
    libGL
    openssl
    pkg-config
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    wayland
    libxkbcommon
    vulkan-loader
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}

# source taken from https://github.com/iced-rs/iced/blob/fe41b3e7608c38ffa136e40c207d068a2b805998/DEPENDENCIES.md :)
