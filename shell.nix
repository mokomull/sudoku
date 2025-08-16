let pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/2c9abb11f4780e7954cd76c7d85441003da21fc8.tar.gz")  {}; in 

pkgs.mkShellNoCC {
    packages = with pkgs; [
        nodejs_24
    ];
}
