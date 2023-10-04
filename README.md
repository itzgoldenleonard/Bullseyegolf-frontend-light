# Bullseyegolf light
Bullseyegolf light is a minimalist web (user) front end for bullseyegolf. It's server rendered with a CGI script written in rust. The codebase is intentionally as small and simple as possible, the goal is for it to be as fast as possible, usable on 2G.

bullseyegolf-frontend-light(-user) is the CGI script. bullseyegolf-lighttpd is the ready to use container that packages the CGI script with lighttpd.

# Project status
See [todo.md](./todo.md)

User CGI script: Finished
Packaging with docker: Finished

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
