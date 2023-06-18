#!/usr/bin/env nix-shell
#!nix-shell --pure -i bash
set -euxo pipefail

cargo run

cp res/style/{source,minified}.css
runhaskell downscale.hs 80 140 >> res/style/minified.css

rm -rf res/images/low_res
mkdir res/images/low_res
mogrify -resize 'x555>' -quality 88 -format webp -auto-orient -path res/images/{low_res,hi_res/*}
