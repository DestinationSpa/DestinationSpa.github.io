## Setup

Install the [Nix package manager](https://nixos.org/download). Nix
will download every dependency and setup a reproducible isolated build
environment for you without polluting your system. This will either
work on a native Linux system, [or on
Windows/WSL2](https://nixos.org/download/#nix-install-windows).

## Build

Directly run the `build.sh` script like that:

```
./build.sh
```

You don't need to install anything but **DON'T DO** `bash build.sh`
(you would short-circuit Nix).

## Edit content

Edit `res/content.ron`, then build. You might get an ugly parsing
error, usually the line number is enough to figure out what you
fucked-up in the content file. If `res/content.ron` is well-formed,
the `index.xhtml` file will be updated; the whole website is in this
single file (except for the font, the style-sheet, and the images).

You can add **full resolution** images in `res/images/hi_res`, and
reference them in `res/content.ron` (take a look at existing images
and references). The build script will downscale and compress your
images and put the results in `res/images/low_res`.

IDs are there so that an URL to a specific category or benefit always
stays the same across updates of the website. Therefore you should not
modify an ID, they are permanent. If you add a new category or
benefit, just assign it a previously unused ID. Benefits IDs are not
global: they are only unique per category.

## Preview

Even though the website is fully static, you still need a web server
to browse it. Just run:

```
./serve.sh
```

You don't need to install anything but again, **DON'T DO** `bash
serve.sh` (you would short-circuit Nix).

Go visit `http://localhost:8080/index.xhtml` and admire the beauty of
this beauty salon's website.

## Gandi
DNS name location : https://account.gandi.net/
