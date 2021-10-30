# Leuchtkraft
Leuchtkraft is my tiny declarative Programming Language. It can run in two modes: Interpretation and Compilation.
Like Prolog, it is a logical programming language. To be specific, its a inductive logical language.

## Interpretation Mode
Interpretation Mode is meant for debugging your program. It can provide faster feedback loops when writing code that Compile mode
but will run slower than the compiled version. Since the Leuchtkraft interpreter is written in Rust, it works on all modern architectures.

## Compilation Mode
Compilation mode will compile the program into LLVM bytecode, taking longer to build but increasing runtime performance. Since Leuchtkraft
compiles to LLVM, it will run on all modern architectures as well.

The concept of two different execution mode was shamelessly stolen from [Porth](https://github.com/tsoding/porth).

