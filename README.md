# cipherdogs-os-art
Cipher Dogs operation system powered by Rust

# Run

### QEMU
```sh
bootimage run
```

### QEMU + serial port
```sh
bootimage run -- -serial mon:stdio
bootimage run -- -serial mon:stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04
```

```sh
bootimage run -- \
    -serial mon:stdio \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -display none
```

### Integration testing
```sh
bootimage test
```
