{
  description = "Toolchain for dataset building";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs = { self, nixpkgs } @inputs:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    packages = with pkgs; [
      temurin-bin-21
      ant
      gzip
      librdf_raptor2
      bash
      perl
      curl
      gnused
      xz
      libuuid
      glibcLocales
      util-linuxMinimal
    ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      inherit packages;

      environment = {
        LOCALE_ARCHIVE = "${pkgs.glibcLocales}/lib/locale/locale-archive";
        LANG = "C.UTF-8";
        LC_ALL = "C.UTF-8";
      };
    };
    packages.${system}.default = pkgs.buildEnv {
      name = "dataset-toolchain";
      paths = packages;

      postBuild = ''
        mkdir -p $out/etc/profile.d

        cat > $out/etc/profile.d/locale.sh <<EOF
export LANG=C.UTF-8
export LC_ALL=C.UTF-8
export LOCALE_ARCHIVE=${pkgs.glibcLocales}/lib/locale/locale-archive
EOF
      '';
    };
  };
}
