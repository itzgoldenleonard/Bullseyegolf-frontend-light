{ pkgs ? import <nixpkgs> { system = "x86_64-linux"; } }:

let 
    lighttpdConfig = pkgs.writeText "lighttpd.conf" ''
        server.modules += ("mod_openssl", "mod_setenv", "mod_cgi", "mod_access")
        server.document-root = "${lighttpdWebRoot}"
        server.port = 4430
        server.name = "localhost"
        url.access-deny = ("~", ".inc")
        static-file.exclude-extensions = (".cgi")
        server.max-fds = 1024
        setenv.set-environment = ( "SERVER_URL" => "${bullseyegolfApiServer}" )
        server.follow-symlink = "enable"
    '';
    #lighttpdWebRoot = "/var/webroot";
    lighttpdWebRoot = (gitRepo + "/server/document-root");
    bullseyegolfApiServer = "https://api.bullseyegolf.org";
    gitRepo = pkgs.fetchFromGitHub {
        owner = "itzgoldenleonard";
        repo = "bullseyegolf-frontend-light";
        rev = "eba1714ae47baa1a881c4b9c851c68efd7e63e50";
        sha256 = "sha256-ngzzR+WXGnObh+hIMvn6odr/3riSmKBuEuL01IiClG8=";
    };
in pkgs.dockerTools.buildImage {
    name = "bullseyegolf-lighttpd";
    tag = "latest";
    copyToRoot = pkgs.buildEnv {
        name = "image-root";
        paths = [ pkgs.lighttpd pkgs.bashInteractive pkgs.coreutils ];
        pathsToLink = [ "/bin" ];
    };
    runAsRoot = ''
        mkdir -p /var/tmp
    '';
    config = {
        Cmd = [ "/bin/lighttpd" "-D" "-f" lighttpdConfig ];
        ExposedPorts = { "4430/tcp" = {}; };
    };
}

# https://nixos.org/manual/nixpkgs/stable/#sec-pkgs-dockerTools
# https://github.com/moby/moby/blob/daa4618da826fb1de4fc2478d88196edbba49b2f/image/spec/v1.2.md
# https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/docker/examples.nix
