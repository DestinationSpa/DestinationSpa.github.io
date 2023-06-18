#!/usr/bin/env nix-shell
#!nix-shell --pure -i bash
set -euxo pipefail

cargo fmt
prettier --write res/style/source.css
