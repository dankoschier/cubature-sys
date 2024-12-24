# [Cubature](https://github.com/stevengj/cubature) bindings for [Rust](https://www.rust-lang.org/)

This is a `*-sys` crate to make the cubature package for **adaptive multidimensional integration** (*cubature*) of **vector-valued integrands** over **hypercubes**, written by
[Steven G. Johnson](http://math.mit.edu/~stevenj), available.

## Contributing

This repo uses git submodules. Make sure you clone with

```sh
git clone --recursive
```

or instead run:

```sh
git submodule update --init
```

Unless the library location is specified using `CUBATURE_INCLUDE_DIR` and `CUBATURE_LIB_DIR` the library needs to be built from source.
Building from source requires to have `cmake` installed.