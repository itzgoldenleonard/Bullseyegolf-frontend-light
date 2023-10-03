{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, rustPlatform ? pkgs.rustPlatform
, fetchFromGitHub ? pkgs.fetchFromGitHub }:

let 
    gitRepo = pkgs.fetchFromGitHub {
        owner = "itzgoldenleonard";
        repo = "bullseyegolf-frontend-light";
        rev = "eba1714ae47baa1a881c4b9c851c68efd7e63e50";
        sha256 = "sha256-ngzzR+WXGnObh+hIMvn6odr/3riSmKBuEuL01IiClG8=";
    };
in
rustPlatform.buildRustPackage rec { 
    #name = "bullseyegolf-frontend-light";
    version = "0.1.0";
    pname = "bullseyegolf-frontend-light";
    src = (gitRepo + "/user");
    cargoSha256 = "sha256-6Cddw2Zk8q4ESqBANLWgSJbjDh05xI3WVHHwJmkA63k=";
}
