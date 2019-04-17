# build & run

`qemu` and nightly toolchain is needed.

``` bash
$ cargo install bootimage --version "^0.7.1"
$ rustup component add llvm-tools-preview
$ bootimage run
```
