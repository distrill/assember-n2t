# n2t-assembler
Assembler for the hack assembly language, as specified in the [Nand 2 Tetris book](https://www.nand2tetris.org/).    
These `hack` bin files can be run with the CPU emulator provided to accompany the aforementioned book.

### usage
Accepts an `asm` filename as an argument, and writes the binary code to a `hack` file of the same name.
```
> cargo run Add.asm

assembling Add.asm
written to Add.hack
```

### examples
```
> cat Add.asm

// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/add/Add.asm

// Computes R0 = 2 + 3  (R0 refers to RAM[0])

@2
D=A
@3
D=D+A
@0
M=D
```

```
> cargo run Add.asm

assembling Add.asm
written to Add.hack
```
```
> cat Add.hack

0000000000000010
1110110000010000
0000000000000011
1110000010010000
0000000000000000
1110001100001000
```

```
> cat RectL.asm

// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/06/rect/RectL.asm

// Symbol-less version of the Rect.asm program.

@0
D=M
@23
D;JLE
@16
M=D
@16384
D=A
@17
M=D
@17
A=M
M=-1
@17
D=M
@32
D=D+A
@17
M=D
@16
MD=M-1
@10
D;JGT
@23
0;JMP
```
```
> cargo run RectL.asm

assembling RectL.asm
written to RectL.asm
```
```
> cat RectL.hack

0000000000000000
1111110000010000
0000000000010111
1110001100000110
0000000000010000
1110001100001000
0100000000000000
1110110000010000
0000000000010001
1110001100001000
0000000000010001
1111110000100000
1110111010001000
0000000000010001
1111110000010000
0000000000100000
1110000010010000
0000000000010001
1110001100001000
0000000000010000
1111110010011000
0000000000001010
1110001100000001
0000000000010111
1110101010000111
```
