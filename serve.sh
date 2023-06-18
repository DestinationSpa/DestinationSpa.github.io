#!/usr/bin/env nix-shell
#!nix-shell --pure -i bash
set -euo pipefail

php --server 0.0.0.0:8080
