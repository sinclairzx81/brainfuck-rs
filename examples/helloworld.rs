extern crate brainfuck;

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