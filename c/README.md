# `ltl-actus`

Linear temporal logic to execute the algorithmic contract types unified standard

## Installation

```sh
nix build ./../#cmake
./result/bin/ltl-actus
```

### Building from source

Needs `cmake`, `gnumake`, `glib`, and `pkg-config`. This is all in the `nix develop ./../#c`.

```sh
nix develop ./../#c
mkdir build
cd build
cmake ./../
make
```

### Running tests

stub
