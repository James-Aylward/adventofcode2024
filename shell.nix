with import <nixpkgs> { };
mkShell {
  buildInputs = [
    cargo
    rustc
    gdb
  ];

  nativeBuildInputs = [
  ];

}
