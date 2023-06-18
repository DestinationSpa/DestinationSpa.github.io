#!/usr/bin/env nix-shell
#!nix-shell --pure -i bash
set -euxo pipefail

cargo run
cat index.xhtml | minify --mime='application/xhtml+xml' > index.min
mv index.{min,xhtml}

{ cat res/style/source.css
  runhaskell downscale.hs 80 140
} | minify --mime='text/css' > res/style/minified.css

new=($(cd res/images/hi_res; cat $(ls -1 | sort) | sha1sum))
old=$(cat res/images/low_res/originals.sha1 || true)
if [[ "$new" != "$old" ]]; then
    rm -rf res/images/low_res
    mkdir res/images/low_res
    mogrify \
        -resize 'x555>' -quality 88 \
        -format webp -auto-orient \
        -path res/images/{low_res,hi_res/*}
    echo -n "$new" > res/images/low_res/originals.sha1
fi
