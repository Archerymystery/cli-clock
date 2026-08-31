{
  description = "cli clock/stopwacth";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = [
          "x86_64-unknown-linux-gnu"
          "x86_64-apple-darwin"
          "aarch64-apple-darwin"
          "x86_64-pc-windows-msvc"
        ];
        extensions = [
          "rust-src"
          "rustfmt"
          "rust-analyzer"
        ];
      };
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };
      cli-clock = rustPlatform.buildRustPackage {
        pname = "cli-clock";
        version = "1.0.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl ];
      };
    in
    {
      packages.${system} = {
        default = cli-clock;
        my-app = cli-clock;
      };
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl
        ];

        RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
      };
    };
}
