# chip8-emulator
An implementation of CHIP-8 in Rust

### Usage
```shell
cargo run --release <path/to/rom>
```

I've added somes games in the `roms` folder to try out. For example, run:
```shell
cargo run --release roms/breakout.ch8
```

### References
 - [Cowgod's Chip-8 reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
 - [How to write an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
