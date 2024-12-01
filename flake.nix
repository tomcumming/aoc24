{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };
  outputs = { self, nixpkgs }:
    let
      system = "aarch64-darwin";
      pkgs = nixpkgs.legacyPackages."${system}";
    in
    {
      devShells."${system}".default = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.rustfmt
          pkgs.rust-analyzer
        ];
      };
      formatter."${system}" = pkgs.nixpkgs-fmt;
    };
}
