# static-link

The following processes are confirmed with OSX.

raw(?) build commands:

```console
clang -c -o target/debug/native/fib.o c_src/fib.c
ar r target/debug/deps/libfib.a target/debug/native/fib.o
RUSTFLAGS="-L native=$(pwd)/target/debug/deps" cargo run
```

or using [cc](https://crates.io/crates/cc) crate (see `build.rs` file):

```console
cargo build
```
