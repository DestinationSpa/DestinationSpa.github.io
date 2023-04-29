let
  github = {repo, rev, sha256}: fetchTarball {
    url = "https://github.com/${repo}/archive/${rev}.tar.gz";
    inherit sha256;
  };

  nixpkgs = github {
    repo = "nixos/nixpkgs";
    rev = "a45a891"; # nixos-22.11 @ 2023-02-13
    sha256 = "1z4iyfgqsqs90g3xmh21n4ydv88p5s8s0klf1x1cbjx4r7q8h6wf";
  };

  nixpkgs-mozilla = github {
    repo = "mozilla/nixpkgs-mozilla";
    rev = "85eb0ba"; # master @ 2023-02-02
    sha256 = "15a7zd7nrnfgjzs8gq2cpkxg7l3c38jradkxxyaf136kkqhlc0k4";
  };

  pkgs = import nixpkgs {
    overlays = [(import nixpkgs-mozilla)];
  };

  cert = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";

in pkgs.mkShell {

  NIX_SSL_CERT_FILE = cert;
  SSL_CERT_FILE = cert;
  GIT_SSL_CAINFO = cert;

  packages = [
    (pkgs.rustChannelOf {

      channel = "stable";
      date = "2023-01-26"; # see https://forge.rust-lang.org
      sha256 = "sha256-riZUc+R9V35c/9e8KJUE+8pzpXyl0lRXt3ZkKlxoY0g=";

    }).rust

    pkgs.nodePackages.prettier
    pkgs.lightningcss
    pkgs.imagemagick
    pkgs.php
    pkgs.ghc
  ];
}
