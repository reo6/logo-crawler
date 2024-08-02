let
  pkgs = (import (builtins.fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/63dacb46bf939521bdc93981b4cbb7ecb58427a0.zip";
    sha256 = "1lr1h35prqkd1mkmzriwlpvxcb34kmhc9dnr48gkm8hh089hifmx";
  }) { });
  stdenv = pkgs.stdenv;
in pkgs.mkShell rec {
  name = "interview";
  shellHook = ''
    source .bashrc
  '';
  buildInputs = (with pkgs; [
    bashInteractive
    rustc
    cargo
    rustfmt
    clippy
  ]);
}

