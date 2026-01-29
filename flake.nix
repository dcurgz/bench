{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = nixpkgs.legacyPackages.${system};
    naersk' = pkgs.callPackage naersk { };
    inherit (nixpkgs) lib;
    inherit (pkgs) stdenv;
  in
  {
    packages.common = naersk'.buildPackage {
      src = builtins.path { path = ./.; name = "bench"; };
      root = ./common;
      buildInputs = with pkgs; lib.optionals stdenv.isDarwin [ libiconv ];
    };
    packages.gateway = naersk'.buildPackage {
      src = builtins.path { path = ./.; name = "bench"; };
      singleStep = true;
      root = ./gateway;
      # This is a bit of a hack.
      # https://github.com/nix-community/naersk/issues/258
      preBuild = "cd gateway";
      buildInputs = with pkgs; lib.optionals stdenv.isDarwin [ libiconv ];
    };
    packages.cli = naersk'.buildPackage {
      src = builtins.path { path = ./.; name = "bench"; };
      singleStep = true;
      root = ./cli;
      preBuild = "cd cli";
      buildInputs = with pkgs; lib.optionals stdenv.isDarwin [ libiconv ];
    };
  });
}
