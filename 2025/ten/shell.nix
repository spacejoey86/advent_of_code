let
    pkgs = import <nixpkgs> {};
in
pkgs.mkShell rec {
    buildInputs = with pkgs; [
        pkg-config

        z3
        libclang
    ];

    shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
    '';
}
