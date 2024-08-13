{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell."${system}" = pkgs.mkShell rec {
        buildInputs = with pkgs; [ zlib ];
        nativeBuildInputs = with pkgs; [ trunk leptosfmt ];
        LD_LIBRARY_PATH = "${pkgs.lib.strings.makeLibraryPath buildInputs}";
      };
    };
}
