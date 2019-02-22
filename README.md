# Rust integration as a Zephyr application

## How to build

```
# at a directory you can build Zephyr application
# setup Zephyr build environment variables, e.g., using `zephyr-env.sh`
git clone https://github.com/tomoyuki-nakabayashi/zephyr-rust-integration.git
cd zephyr-rust-integration/hello_rust
mkdir build && cd build
cmake -GNinja -DBOARD=qemu_cortex_m3 ..
ninja run
```

Then, you can finally see:

```
***** Booting Zephyr OS zephyr-v1.13.0-3321-g7f956a9 *****
Hello World! qemu_cortex_m3
Hello from Rust.
```

## prerequisite

### Zephyr side

First of all, you must setup Zephyr development environment to build application targeting `qemu_cortex_m3`.
[Zephyr Getting Started Guide](https://docs.zephyrproject.org/latest/getting_started/index.html) will help you to setup the development environment.

I tested this sample using zephyr-sdk-0.9.5.
I recommend you use the same version of zephyr-sdk.

### Rust side

- Rust 1.32.0 stable channel.
- cargo-binutils

```
$ rustup component add llvm-tools-preview

$ cargo install cargo-binutils --vers 0.1.4

$ cargo size -- -version
LLVM (http://llvm.org/):
  LLVM version 8.0.0svn
  Optimized build.
  Default target: x86_64-unknown-linux-gnu
  Host CPU: skylake
```

- cbindgen

```
cargo install cbindgen
```

## Limitations

- Build only for `thumbv7m-none-eabi`.
