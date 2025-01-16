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
            version = "0.1.3";

            src = pkgs.fetchzip {
              stripRoot = false;
              url = "https://github.com/akiomik/mado/releases/download/v${version}/mado-${os}-${arch}.tar.gz";
              sha256 =
                {
                  x86_64-linux = "0irf7c8c4z5h68yrq32xbv6xq7gpbszp76a9zki5iy3g83rnghl9";
                  aarch64-linux = "1z4i8hxbjhg29ldjjq4wdf3k3yp2r0mpcs621zm205ppx21cha7c";
                  x86_64-darwin = "1fdh2sxnv02bnnfa29ci20iyl7krk76dfpivdyzn82378nffwsw5";
                  aarch64-darwin = "1fkvin93z9xjq0mhh04qjfy2vc8n9p4glg7hi333qd79nmzwiv7m";
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
