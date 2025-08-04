# start dev window
dev:
  #! /bin/bash
  nixGLMesa dx serve --platform desktop
  # pnpx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch

# cargo build release binary
db:
  dx bundle

# remove the target dir
dc:
  dx clean

# format rsx files
df:
  dx fmt

# nix build release derivative
nb:
  nix build --log-format internal-json |& nom --json

# nix update flake.lock
nu:
  nix flake update

# nix check flake
nc:
  nix flake check

# nix show flake's info
ns:
  nix flake show

# reload environment
d:
  direnv reload