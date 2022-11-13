let
  github = {repo, rev, sha256}: fetchTarball {
    url = "https://github.com/${repo}/archive/${rev}.tar.gz";
    inherit sha256;
  };

  nixpkgs = github {
    repo = "nixos/nixpkgs";
    rev = "5c211b4"; # nixos-22.05 2022-08-12
    sha256 = "1r6wj98wb16217g6hsk13qwwpx5zwd1nq4fnx6an6ljmv5mq5mc3";
  };

  nixpkgs-mozilla = github {
    repo = "mozilla/nixpkgs-mozilla";
    rev = "0508a66"; # master 2022-07-07
    sha256 = "1nswjmya72g0qriidc2pkl54zn5sg0xp36vdq0ylspca56izivxc";
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
      date = "2022-11-03"; # see https://forge.rust-lang.org
      sha256 = "sha256-DzNEaW724O8/B8844tt5AVHmSjSQ3cmzlU4BP90oRlY=";

    }).rust
  ];
}
