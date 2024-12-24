# [Cubature](https://github.com/stevengj/cubature) bindings for [Rust](https://www.rust-lang.org/)

This is a `*-sys` crate to make package for **adaptive multidimensional integration** (*cubature*) of **vector-valued integrands** over **hypercubes**, written by
[Steven G. Johnson](http://math.mit.edu/~stevenj), available.
It allows to compute integrals of the form:
![n-dimensional integral](vendor/doc/integral.png)

## Contributing

This repo uses git submodules. Make sure you clone with

```sh
git clone --recursive`
```

or instead run:

```sh
git submodule update --init
```