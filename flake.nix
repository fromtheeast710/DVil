{
  outputs = inputs:
    with inputs; let
      system = "x86_64-linux";

      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
          nixgl.overlay
        ];
      };

      rust = pkgs.rust-bin.stable.latest.default;

      comet = {
        lib,
        fetchgit,
        rustPlatform,
      }:
        rustPlatform.buildRustPackage {
          pname = "comet";
          version = "unstable";
          src = fetchgit {
            url = "https://github.com/iced-rs/comet";
            hash = "sha256-54T/v8rOqexV6v5+SEQCpVN3k+ry4DkKrNH+TbUtCSM=";
          };
          cargoHash = "sha256-UGCLJwCyLH5/QjvnI/HQtR04cEaenz167e78LtwSzsQ=";

          buildInputs = with pkgs; [
            openssl
          ];

          nativeBuildInputs = with pkgs; [pkg-config];
        };

      # TODO: rewrite
      builder = {
        lib,
        rustPlatform,
      }: let
        toml = (lib.importTOML ./Cargo.toml).package;
      in
        rustPlatform.buildRustPackage {
          inherit (toml) version;

          pname = toml.name;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [pkg-config];

          meta.mainProgram = "dvil";
        };
    in
      with pkgs; {
        packages.${system} = {
          dvil = callPackage builder {};
          default = self.packages.${system}.dvil;
        };

        devShells.${system}.default = mkShellNoCC rec {
          packages = [
            rust
            nickel
            watchexec
            nixgl.nixGLMesa

            (callPackage comet {})
          ];

          buildInputs =
            [
              libclang
              expat
              fontconfig
              freetype
              freetype.dev
              pkg-config
              wayland
              libxkbcommon
              # libGL
            ]
            ++ (with xorg; [
              libX11
              libXcursor
              libXi
              libXrandr
            ]);

          env = {
            RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";
            LIBCLANG_PATH = "${libclang.lib}/lib";
            LD_LIBRARY_PATH =
              builtins.foldl' (a: b: "${a}:${b}/lib") "${vulkan-loader}/lib" buildInputs;
          };
        };
      };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixgl.url = "github:nix-community/nixGL";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
}
