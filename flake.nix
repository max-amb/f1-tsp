{
  description = "F1-tsp is a project to calculate the fastest distance around all of the f1 locations.";

  inputs = {
    nixpkgs.url = "github:Nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      lib = nixpkgs.lib;
    in
    {
      defaultPackage.x86_64-linux = pkgs.rustPlatform.buildRustPackage {
        pname = "f1-tsp";
        version = "0.2";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        buildInputs = [ pkgs.graphviz ];

        preInstall = ''
          mkdir -p $out/data
          cp ./data/f1-locations.json $out/data/f1-locations.json
        '';
        
        meta = {
          description = "F1-tsp is a project to calculate the fastest distance around all of the f1 locations.";
          homepage = "https://github.com/max-amb/f1-tsp";
          license = lib.licenses.mit;
          maintainers = with lib.maintainers; [ max-amb ];
          platforms = lib.platforms.linux;
        };
      };
    };
}
