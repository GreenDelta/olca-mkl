# olca-mkl

This is an experimental project that links the Intel Math Kernel (MKL) as calculation library for openLCA. The chart below shows a performance comparison with UMFPACK for fully solving `A * x = b` for an ecoinvent 3.9 matrix including JNI overhead for communicating between JVM and native memory:

![Performance comparison UMFPACK vs MKL](./performance.png)

## Building

The build scripts are written in Dart so that you need to have a current version
of Dart (3.x) installed as well as the `archive` library. To install the
library, run:

```bash
dart pub add archive
```

On macOS with Apple M1/M2, one will need to set the Rust toolchain to
`stable-x86_64-apple-darwin`:

```bash
rustup install stable-x86_64-apple-darwin
rustup default stable-x86_64-apple-darwin
```

Then to build:

```bash
cd olca-mkl
dart build.dart
```

This will download the MKL Python package and its dependencies from pypi.org and extract the native libraries into the `bin` folder. It then compiles the bindings for the Java Native Interface (JNI) which are written in Rust against these libraries. (On Windows, it also generates a lib-file with the exported symbols for the linker first). This should then generate a `(lib)olcamkl.*` library in the `bin` folder.

The Java part has an `MKL` class with the native method-bindings and methods for loading the libraries from a folder. The method `MKL.loadFrom(DIR)` will load the libraries from the folder `{DIR}/olca-mkl-x64_v{VERSION}`. The current version is `1` (the next version would be `2`, then `3` etc.) and it is not the version of the MKL but the version of the openLCA JNI bindings for the MKL. The method `MKL.loadFromDefault()` will try to load the libraries from the openLCA default workspace `~/openLCA-data-1.4/olca-mkl-x64_v{VERSION}`. With the methods `MKL.isLibraryDir` and `MKL.isDefaultLibraryDir` you can also test if a directory contains the MKL library folder with the required libraries (useful for the integration in openLCA later).

**Note** that there is only support for `x64` CPUs, e.g.  on macOS M1/2, you need to run a `x64` JVM using the compatibility layer.
