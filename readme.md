# brainfuck-rs

a brainfuck interpreter in rust.

```rust
use brainfuck::Program;

fn main() {
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

brainfuck-rs is a small interpreter for the brainfuck programming language written in Rust. This project was written to 
experiment with hosting a small vm inside of Rust and to explore various threading and io related concepts. This project
was also written for fun.

information on the brainfuck language can be found at [https://en.wikipedia.org/wiki/Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)

This project offered as is for anyone who finds it useful or interesting.

## running examples

```
cargo run --example helloworld

cargo run --example fibonacci

cargo run --example mandelbrot
```