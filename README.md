# cipherdogs-os-art
Cipher Dogs operation system powered by Rust

## Dependencies
```sh
sudo apt install qemu
sudo apt install qemu-system-x86
cargo install cargo-xbuild
cargo install bootimage
rustup default nightly
rustup component add rust-src
rustup component add llvm-tools-preview
```

## Run

### QEMU
```sh
cargo xrun
```

### Integration testing
```sh
cargo xtest
```
