#!/usr/bin/env nix-shell
#!nix-shell --pure -i bash
set -euo pipefail

cargo fmt
prettier --write res/style/source.css
