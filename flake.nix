{
  description = "Corplink Rust client";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        {
          devShell = pkgs.mkShell {
            hardeningDisable = [ "all" ];

            nativeBuildInputs = with pkgs;[
              pkg-config
              llvmPackages.clang
            ];

            buildInputs = with pkgs; [
              go_1_20
              rustup
              openssl
            ];

            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          };
        }
      );
}
