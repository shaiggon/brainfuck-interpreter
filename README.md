# A Simple Brainfuck Interpreter in Rust

This is a toy project implementing an interpreter for the Brainfuck language in Rust.
The implementation is done only by using the [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck) description of the language as reference.

## Usage

Build the interpreter with 

```
cargo build --release
```

Then use the executable to run any brainfuck program on the command line. The path to the program source is provided as the first argument to the interpreter.

```
target/release/brainfuck-rust brainfuck-examples/hello_world.bf
```

Any input read with `,` is read from stdin. 
