# Hello Driver Written in Rust

## prerequisites

- Zephyr (1.13)
- Zephyr SDK (0.9.5)
- Rust toolchain (tested version: 1.33.0 stable)
  - thumbv7m-none-eabi target
- cargo-binutils (tested version: 0.1.4)

## Build && Run

```
mkdir build && cd $_
cmake -GNinja -DBOARD=qemu_cortex_m3 ..
ninja run
```

You'll see:

```
[0/1] To exit from QEMU enter: 'CTRL+a, x'[QEMU] CPU: cortex-m3
qemu-system-arm: warning: nic stellaris_enet.0 has no peer
Hello from My Driver!

***** Booting Zephyr OS 1.13.99 *****
Hello from Rust!
```