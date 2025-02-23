# Google CP-SAT solver Rust bindings [![](https://img.shields.io/crates/v/cp_sat.svg)](https://crates.io/crates/cp_sat) [![](https://docs.rs/cp_sat/badge.svg)](https://docs.rs/cp_sat)

Rust bindings to the Google CP-SAT constraint programming solver.

To use this library, you need a C++ compiler and an installation of
google or-tools library files.

The environment variable `ORTOOLS_PREFIX` is used to find include
files and library files. If not setted, `/opt/ortools` will be added
to the search path (classical search path will also be used).

## Setup

### Build or-tools from source

Clone the [OR-Tools](https://github.com/google/or-tools) github repository
```zsh
git clone https://github.com/google/or-tools.git
```

For Mac M1s run the following commands:

```zsh
cd or-tools
CMAKE_HOST_SYSTEM_PROCESSOR=arm64 GCC_PREFIX=aarch64 TARGET=aarch64 cmake -S . -Bbuild_arm64 -DBUILD_DEPS=ON -DCMAKE_APPLE_SILICON_PROCESSOR=arm64
CMAKE_HOST_SYSTEM_PROCESSOR=arm64 GCC_PREFIX=aarch64 TARGET=aarch64 cmake --build build_arm64 --config Release -j -v
```

### Attempts at making it work

When trying to run the project with `cargo run`, I was first getting the following error:

```zsh
  cargo:warning=In file included from src/cp-sat-wrapper.cc:3:
  cargo:warning=/usr/local/include/ortools/sat/cp_model.h:50:10: fatal error: 'absl/container/flat_hash_map.h' file not found
  cargo:warning=   50 | #include "absl/container/flat_hash_map.h"
  cargo:warning=      |          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
  cargo:warning=1 error generated.
```

Hence, I tried first to compile the file `src/cp-sat-wrapper.cc` by hand using the following commands:

```zsh
g++ -arch arm64 src/cp-sat-wrapper.cc -fPIC -o cp-sat-wrapper.o -lc++ -std=c++20 -lc -L/Users/antoinepouillaude/Workspace/or-tools/build_arm64/lib -I/Users/antoinepouillaude/Workspace/or-tools -v
```

Even after the compilation steps above I was still getting errors:
```zsh
Undefined symbols for architecture arm64:
  "_FLAGS_time_limit_use_usertime", referenced from:
```

I also encountered issue with protobuf version.

### What worked

I downloaded an already compiled `.tar.gz` from the github repository of the Google OR Tools, I ran the following command:
```zsh
ORTOOLS_PREFIX="/Users/antoinepouillaude/Downloads/or-tools-9.11" cargo build
```

And with that it worked !

