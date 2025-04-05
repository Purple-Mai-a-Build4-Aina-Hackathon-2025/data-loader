{
  description = "Rust dev environment";

  inputs = {nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";};

  outputs = {
    self,
    nixpkgs,
  }: let
    allSystems = ["x86_64-linux" "aarch64-darwin"];
    forAllSystems = fn:
      nixpkgs.lib.genAttrs allSystems
      (system: fn {pkgs = import nixpkgs {inherit system;};});
  in {
    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          # Probably don't need this but keeping it around I guess
          # pkg-config
          # openssl
        ];
      };
    });
  };
}
