{
  description = "Virtual use space driver for boox devices";
inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, systems, rust-overlay }: let
    overlays = [ rust-overlay.overlays.default ];
    forAllSystems = fn:
      nixpkgs.lib.genAttrs (import systems) (system: fn {
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inherit system;
      });
  in {
    devShells = forAllSystems ({ pkgs, system }: with pkgs; {
      default = mkShell {
        buildInputs = [
          rust-bin.nightly.latest.default
          android-tools
        ];

        nativeBuildInputs = [
          pkg-config
        ];

        shellHook = ''
        exec ash
        '';
      };
    });

    packages = forAllSystems ({ pkgs, system }: with pkgs; rec {
      vboox-bin = rustPlatform.buildRustPackage {
        pname = "vboox";
        version = "0.1.0";

        src = ./.;
        cargoLock = {
            lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = [ pkg-config ];
        buildInputs = [];
      };
      default = let
        inherit vboox-bin;
        builder-script = pkgs.writeShellScriptBin "builder" ''
          $busybox cp -r $run_vbooxd $out
        '';
      in derivation rec {
        name = "vbooxd";

        busybox = "${pkgs.busybox}/bin/busybox";
        adb = "${pkgs.android-tools}/bin/adb";

        vbooxd = pkgs.writeShellScriptBin "vbooxd" (builtins.readFile ./vbooxd.sh);
        run_vbooxd = pkgs.writeShellScriptBin "run-vbooxd" ''
          set -e

          export PATH=$PATH:${pkgs.android-tools}/bin
          export vboox_bin_path=${vboox-bin}/bin/vboox
          ${busybox} setsid sh -c "exec ${vbooxd}/bin/vbooxd $1" &
        '';

        builder = "${builder-script}/bin/builder";
        inherit system;
      };
    });

    overlays.default = { final, prev }: {
      run-vbooxd = self.packages.${final.stdenv.system}.default;
    };
  };
}
