{ pkgs ? import <nixpkgs> { system = "x86_64-linux"; } }:

let 
    lighttpdConfig = pkgs.writeText "lighttpd.conf" ''
        server.modules += ("mod_openssl", "mod_setenv", "mod_cgi", "mod_access", "mod_alias", "mod_proxy", "mod_redirect")
        server.port = 443
        server.name = "localhost"
        ssl.engine = "enable"
        ssl.pemfile = "/certs/fullchain.pem"
        url.access-deny = ("~", ".inc")
        server.max-fds = 1024
        server.follow-symlink = "enable"
        server.document-root = "${lighttpdWebRoot}"
        setenv.add-response-header = ("Referrer-Policy" => "same-origin")

        # Bullseyegolf light
        $HTTP["host"] =^ "light" {
            $HTTP["url"] =$ "/u" {
                setenv.set-environment = ( "SERVER_URL" => "${apiUrl}" )
                alias.url = (
                    "/u" => "/bin/bullseyegolf-frontend-light-user"
                )
                cgi.assign = ("" => "")
            }
        } else {
            proxy.server  = ( "" =>
                (( "host" => "${apiHost}", "port" => ${apiPort} ))
            )
            # The following doesn't seem to be neccessary locally
            # but cloudflare might need it
            proxy.forwarded = (
                "for"          => 1,
                "proto"        => 1,
                "host"         => 1,
            )
        }

    '';
    lighttpdWebRoot = (gitRepo + "/server/document-root");
    apiHost = "api";
    apiPort = "8000";
    apiUrl = "http://${apiHost}:${apiPort}";
    gitRepo = pkgs.fetchFromGitHub {
        owner = "itzgoldenleonard";
        repo = "bullseyegolf-frontend-light";
        rev = "37bb592611e0bb280ee9e6f61d776456ef5a54a2";
        sha256 = "sha256-yuzz9kPZiZcOAXgUmkd6cRIELTlZaLaJ/A3ejWOc1tI=";
    };
    bullseyegolfLight = import ./user/default.nix { inherit pkgs; };
in pkgs.dockerTools.buildImage {
    name = "bullseyegolf-lighttpd";
    tag = "latest";
    copyToRoot = pkgs.buildEnv {
        name = "image-root";
        #paths = [ pkgs.lighttpd pkgs.bashInteractive pkgs.coreutils bullseyegolfLight ]; # for debugging
        paths = [ pkgs.lighttpd bullseyegolfLight ];
        pathsToLink = [ "/bin" ];
    };
    runAsRoot = ''
        mkdir -p /var/tmp
    '';
    config = {
        Cmd = [ "/bin/lighttpd" "-D" "-f" lighttpdConfig ];
        ExposedPorts = { "443/tcp" = {}; };
    };
}

# https://nixos.org/manual/nixpkgs/stable/#sec-pkgs-dockerTools
# https://github.com/moby/moby/blob/daa4618da826fb1de4fc2478d88196edbba49b2f/image/spec/v1.2.md
# https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/docker/examples.nix
