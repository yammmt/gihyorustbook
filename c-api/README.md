# c-api

```console
cargo build
clang main.c -L target/debug -lc_api
# my OSX works without the following line
# export DYLD_LIBRARY_PATH=./target/debug:$DYLD_LIBRARY_PATH
./a.out
```

```console
> ls target/debug/*.dylib
target/debug/libc_api.dylib
```
