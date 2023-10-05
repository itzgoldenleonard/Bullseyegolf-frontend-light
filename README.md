# Bullseyegolf light

Bullseyegolf light is a minimalist web (user) front end for bullseyegolf. It's server rendered with a CGI script written in rust. The codebase is intentionally as small and simple as possible, it needs to be usable on 2G.

bullseyegolf-frontend-light(-user) is the CGI script. bullseyegolf-lighttpd is the ready to use container that packages the CGI script with lighttpd.


# Project status

This project is finished.

If there haven't been any commits to this repo in a while that's not because the project is dead. There are no new features planned, the design is finished, the code is well optimized and tested, so bugs are unlikely and the performance and code quality are about as good as they can get. If you do find any bugs please report them, I will fix any that there are. The dependencies and features used are also very unlikely to be deprecated since the script is written in rust and almost all web features used are old and settled.

# Building the docker container

1. Download and install nix

<https://nixos.org/download>

2. Clone the git repo

```sh
git clone --depth=1 https://github.com/itzgoldenleonard/Bullseyegolf-frontend-light.git
cd Bullseyegolf-frontend-light
```

3. Build docker image

```sh
nix-build docker.nix -o build/bullseyegolf-lighttpd
```

4. See [the bullseyegolf-server repo](https://github.com/itzgoldenleonard/BullseyeGolf-server/blob/main/docker-compose-lighttpd.yml) for an example of how to deploy it


# Developing

## Dependencies

- rustc
- cargo
- cc (clang)
- lighttpd
- Something to generate a TLS certificate

1. Clone the git repo

```sh
git clone --depth=1 https://github.com/itzgoldenleonard/Bullseyegolf-frontend-light.git
cd Bullseyegolf-frontend-light
```

2. Generate TLS certificate

```sh
openssl req -x509 -newkey rsa:4096 -subj "/" -nodes -keyout fullchain.pem -out fullchain.pem -days 9999
```

3. Customize the test.conf lighttpd config

If your username is not 'ava' you'll need to change the paths to point to the correct places
You'll also have to chose whether to use debug or release binaries for the CGI script
You might need to change the server url depending on what you're testing

Start lighttpd for testing with:

```sh
nix-shell -p lighttpd --run lighttpd -D -f test.conf
```

# CGI script documentation

The documentation for the CGI script can be built using

```sh
cd user/
cargo d
```

It will be located in [`user/target/doc/bullseyegolf_frontend_light_user/index.html`](./user/target/doc/bullseyegolf_frontend_light_user/index.html)
