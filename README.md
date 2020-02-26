# chip8-emulator
An implementation of CHIP-8 in Rust

### Usage
```shell
cargo run --release <chip8-rom>
```

The implementation is still a work in progress; not all the opcodes have been implemented yet, but enough have been implemented to make a good number of ROMs still work. For example, run:
```shell
cargo run --release roms/zero.ch8
```
