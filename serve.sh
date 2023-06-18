#!/usr/bin/env nix-shell
#!nix-shell --pure -i bash
set -euxo pipefail

php --server 0.0.0.0:8080
