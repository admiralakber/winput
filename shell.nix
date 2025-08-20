{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    pkg-config
    cacert
    gtk4
    glib
    cairo
    pango
    gdk-pixbuf
    harfbuzz
    wayland
    fontconfig
    nerd-fonts.inconsolata
  ];
  SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
  NIX_SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
  shellHook = ''
    echo "winput dev shell: cargo run --quiet"
  '';
}
