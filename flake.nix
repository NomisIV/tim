{
  description = "tim - Terminal Interface eMail Client";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, naersk }: let
    # The architecture for the build
    arch = "x86_64-linux";

    # Which package set to use
    pkgs = nixpkgs.legacyPackages."${arch}";

    dependencies = with pkgs; [
      openssl
      pkg-config
    ];

    # The binary program
    tim = naersk.lib."${arch}".buildPackage {
      pname = "tim";
      root = ./.;
      buildInputs = dependencies;
      doCheck = true;
    };

  in {
    defaultPackage."${arch}" = tim;

    devShell."${arch}" = pkgs.mkShell {
      buildInputs = with pkgs; [ rustc cargo rustfmt ] ++ dependencies;
    };
  };
}
