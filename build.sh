#!/usr/bin/env nix-shell
#!nix-shell --pure -i bash
set -euo pipefail

cargo run
cat index.xhtml | minify --mime='application/xhtml+xml' \
                         --xml-keep-whitespace > index.min
mv index.{min,xhtml}

{ cat res/style/source.css
  runhaskell downscale.hs 80 140
} | minify --mime='text/css' > res/style/minified.css

names=$(ls -1 res/images/hi_res | sort)
new=($(cd res/images/hi_res; { echo $names; cat $names; } | sha1sum))
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
