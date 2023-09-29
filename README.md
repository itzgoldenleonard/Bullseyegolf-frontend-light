# Bullseyegolf light
Bullseyegolf light is a minimalist web (user) front end for bullseyegolf. It's server rendered with a CGI script written in rust. The codebase is intentionally as small and simple as possible, the goal is for it to be as fast as possible, usable on 2G.

# Project status
See [todo.md](./todo.md)

User CGI script: Finished
Packaging with docker: WIP

# Building for release

1. Compile the CGI script

```sh
cd user
nix-shell
cargo b --release
```
