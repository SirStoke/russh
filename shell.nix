let
  pkgs = import <nixpkgs> { config = {}; overlays = []; };
in
pkgs.mkShellNoCC {
  packages = with pkgs; [
      rustup
      pkgsCross.wasi32.stdenv.cc
      pkgs.pkgsCross.wasi32.llvmPackages.lld
      binaryen
      wabt
      wasmtime
      wasmedge
      rust-analyzer
      python3
      websocat
      aws-lc
  ];

  shellHook = ''
    export PATH="$HOME/.cargo/bin:$PATH"
    if command -v rustup >/dev/null 2>&1; then
      toolchain_bin="$(dirname "$(rustup which cargo)")"
      export PATH="$toolchain_bin:$PATH"
      export CARGO="$toolchain_bin/cargo"
      export RUSTC="$(rustup which rustc)"
      export RUSTDOC="$(rustup which rustdoc)"
    fi
    echo "CC=$CC"
  '';
}
