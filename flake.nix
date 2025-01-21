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
            version = "0.1.5";

            src = pkgs.fetchzip {
              stripRoot = false;
              url = "https://github.com/akiomik/mado/releases/download/v${version}/mado-${os}-${arch}.tar.gz";
              sha256 =
                {
                  x86_64-linux = "0cmbnpr3v0mkmk521vr3fv47nr5p9fh3aq0zr2dmlchk1x39gr3g";
                  aarch64-linux = "1gj95dvvdds2hjf5741pk8xs1g9j0xq2f65j6774kgv54ran7dpc";
                  x86_64-darwin = "0ym997dg1i1n5d13kr1qvcsfbqksaly68y6sim4awbk7rl1i1lid";
                  aarch64-darwin = "05zp4xc1b6skmrsridp850njvwfa7nym46kyf0cg3qnwl6sq3m40";
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
