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
          sqlx-cli
          pkg-config
          openssl
        ];
      };
    });
  };
}
