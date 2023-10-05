{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, rustPlatform ? pkgs.rustPlatform
, fetchFromGitHub ? pkgs.fetchFromGitHub }:

let 
    gitRepo = fetchFromGitHub {
        owner = "itzgoldenleonard";
        repo = "bullseyegolf-frontend-light";
        rev = "37bb592611e0bb280ee9e6f61d776456ef5a54a2";
        sha256 = "sha256-yuzz9kPZiZcOAXgUmkd6cRIELTlZaLaJ/A3ejWOc1tI=";
    };
in
rustPlatform.buildRustPackage rec { 
    #name = "bullseyegolf-frontend-light";
    version = "0.1.0";
    pname = "bullseyegolf-frontend-light";
    src = (gitRepo + "/user");
    cargoSha256 = "sha256-K7HOjqfhTdChJ8jsOyL7eZztcV5jBla7yoMIhT0ltp0=";
}
