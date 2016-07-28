# brainfuck-rs

a rust implementation of the brainfuck programming language.

```rust
use brainfuck::Program;

fn main() {

    // Hello World!
    let mut program = Program::create("
    ++++++++[>++++[>++>+++>+++>+<<<<-]
    >+>+>->>+[<]<-]>>.>---.+++++++..++
    +.>>.<-.<.+++.------.--------.>>+.>
    ++.
    ").unwrap();

    program.stdin  (Box::new (|| 0u8 ));
    program.stdout (Box::new(|b| print!("{}", b as char))); 
    program.run().unwrap();
}
```

## overview

brainfuck-rs is a Rust implementation of the brainfuck programming language written
as a experiment embedding simple programs within rust.

Information on the brainfuck language can be found at [https://en.wikipedia.org/wiki/Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)

Project offered as is for anyone who finds it useful.

## running examples

```
cargo run --example helloworld
cargo run --example fibonacci
cargo run --example mandelbrot
```