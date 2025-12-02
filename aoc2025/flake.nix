# echo use flake > .envrc; direnv allow
# nix develop
{
  description = "A nix flake with some dev toolchains";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f: nixpkgs.lib.genAttrs supportedSystems (system: f { pkgs = import nixpkgs { inherit system; }; });
    in
    {
      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default =
            pkgs.mkShell.override
              {
                # Override stdenv in order to change compiler:
                # stdenv = pkgs.clangStdenv;
              }
              {
                packages =
                  (with pkgs; [
                    clang
                    clang-tools
                    gcc
                    gdb
                    lcov
                    llvmPackages_21.libllvm
                    lld
                    lldb
                  ])
                  ++ (with pkgs; [
                    rustc
                    cargo
                    clippy
                    rust-analyzer
                  ])
                  ++ (with pkgs; [ typst ])
                  ++ (with pkgs; [
                    nixpkgs-fmt
                  ]);
                env = {
                  # for rust-analyzer
                  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
                };
              };
        }
      );
    };
}

