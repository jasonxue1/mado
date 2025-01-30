{
  description = "Flake for building Mado packages";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        os = if pkgs.stdenv.hostPlatform.isDarwin then "macOS" else "Linux-gnu";
        arch = if pkgs.stdenv.hostPlatform.isAarch64 then "arm64" else "x86_64";

      in
      {
        packages = {
          mado = pkgs.stdenv.mkDerivation rec {
            pname = "mado";
            version = "0.2.0";

            src = pkgs.fetchzip {
              stripRoot = false;
              url = "https://github.com/akiomik/mado/releases/download/v${version}/mado-${os}-${arch}.tar.gz";
              sha256 =
                {
                  x86_64-linux = "1a3qd1h0fgb0hixl8drsn274mw52b56v7xkn378ciwayxgf5r430";
                  aarch64-linux = "0ibdrwvjg600m5rhcqcp841ailk8wa4adk69zld1l1jdjhlzbg36";
                  x86_64-darwin = "1hbvxs6fg6fafsvamvz7i3fkv9zhl0igikj3smqq3p98ypj9njrm";
                  aarch64-darwin = "0rpfqn10q1j9jlr5qi21vnm94vw0glkbjpi7xykz09p3k68j3h8m";
                }
                .${system} or (throw "unsupported system ${system}");
            };

            installPhase = ''
              mkdir -p $out/bin
              cp mado $out/bin/
            '';

            meta = with pkgs.lib; {
              homepage = "https://github.com/akiomik/mado";
              description = "A fast Markdown linter written in Rust";
              license = licenses.asl20;
              sourceProvenance = [ sourceTypes.binaryNativeCode ];
            };
          };
          default = self.packages.${system}.mado;
        };
        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
