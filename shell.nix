let
  github = {repo, rev, sha256}: fetchTarball {
    url = "https://github.com/${repo}/archive/${rev}.tar.gz";
    inherit sha256;
  };

  nixpkgs = github {
    repo = "nixos/nixpkgs";
    rev = "c7ff1b9"; # nixos-23.05 @ 2023-06-16
    sha256 = "sha256:0bq722ick80qwg1k1zv1021b2x7mlwsxv10y999mh22gk7vdfmi7";
  };

  nixpkgs-mozilla = github {
    repo = "mozilla/nixpkgs-mozilla";
    rev = "e6ca26f"; # master @ 2023-05-19
    sha256 = "sha256:15r06ddyx3i7fnnpz3kp68gq22hq87yl3pnwfbybm74drw80k5s9";
  };

  pkgs = import nixpkgs {
    overlays = [(import nixpkgs-mozilla)];
  };

  cert = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";

in pkgs.mkShell {
  NIX_SSL_CERT_FILE = cert;
  SSL_CERT_FILE = cert;
  GIT_SSL_CAINFO = cert;

  nativeBuildInputs = [
    (pkgs.rustChannelOf {
      channel = "stable";
      date = "2023-06-01"; # see https://forge.rust-lang.org
      sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc=";
    }).rust

    pkgs.nodePackages.prettier
    pkgs.imagemagick
    pkgs.minify
    pkgs.ghc # 💖
    pkgs.php
  ];
}
