# chip8-emulator
An implementation of the CHIP-8 interpreter

### Usage
```shell
cargo run --release <path/to/rom>
```

I've added somes games in the `roms` folder to try out. For example, to play the classic game [Breakout](https://en.wikipedia.org/wiki/Breakout_clone):
```shell
cargo run --release roms/breakout.ch8
```

#### Keyboard Input
CHIP-8 has a 16-key keypad, denoted in hex (`0-F`). The keypad is mapped to the keyboard as follows:
<table>
<tr><th>Chip-8 Keypad</th><th>Keyboard Mappings</th></tr>
<tr><td>

|||||
 |--|--|--|--|
|1|2|3|C|
|4|5|6|D|
|5|6|7|E|
|A|0|B|F|

</td><td>

|||||
 |--|--|--|--|
|1|2|3|4|
|Q|W|E|R|
|A|S|D|F|
|Z|X|C|V|

</td></tr> </table>

For example, to play `breakout.ch8`, the controls are 4 and 6 to move the paddle left and right (Q and E respectively on the keyboard).

## References
 - [Cowgod's Chip-8 reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
 - [How to write an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
