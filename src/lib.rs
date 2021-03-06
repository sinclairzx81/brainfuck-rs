/*--------------------------------------------------------------------------

brainfuck-rs - a brainfuck interpreter in rust.

The MIT License (MIT)

Copyright (c) 2016 Haydn Paterson (sinclair) <haydn.developer@gmail.ins>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

---------------------------------------------------------------------------*/

#[derive(Debug)]
pub enum Error {
    InvalidProgram,
    InvalidInstruction,
    InsPtrOutOfBounds(i32),
    MemPtrOutOfBounds(i32),
}

/// brainfuck program
pub struct Program {
    ins       : Vec<char>,
    mem       : Vec<u8>,
    ins_ptr   : usize,
    mem_ptr   : usize,
    io_in     : Box<FnMut()   -> u8 + Send + 'static>,
    io_out    : Box<FnMut(u8) -> () + Send + 'static>
}
/// user friendly implementation.
impl Program {

    /// parses the given string and returns a brainfuck instructions.
    ///
    /// #Example
    /// ```
    /// let instructions = Program::parse("++++++.").unwrap();
    /// let program = Program::new(instructions, vec![0; 30000]);
    /// ```
    pub fn parse(code: &'static str) -> Result<Vec<char>, Error> {
        let ins = code.chars().map(|c| c as char).filter(|c| {
            match *c {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
                _ => false
            }
        }).collect::<Vec<char>>();
        let valid = ins.iter().fold(0, |acc, c| {
            match *c { '[' => acc + 1, ']' => acc - 1, _ => acc }
        });
        match valid { 0 => Ok(ins), _ => Err(Error::InvalidProgram) }
    }

    /// creates a program from the given program string.
    ///
    /// #Example
    /// ```
    /// let mut program = Program::create("+++++++.").unwrap();
    /// ```
    pub fn create(code: &'static str) -> Result<Program, Error> {
        let ins = try!(Program::parse(code));
        let mem = vec![0; 30000];
        Ok(Program::new(ins, mem))
    }   
}

/// brainfuck implementation.
impl Program  {

    /// creates a new program from the given instructions and memory buffer.
    ///
    /// #Example
    /// ```
    /// let mut program = Program::new(vec!['.', ','], vec![0; 30000]);
    /// ``` 
    pub fn new (ins: Vec<char>, mem: Vec<u8>) -> Program {
        Program {
            ins     : ins,
            mem     : mem,
            ins_ptr : 0,
            mem_ptr : 0, 
            io_out  : Box::new(|_| {}),
            io_in   : Box::new(|| 0u8) 
        }
    }
    /// sets this programs stdin handler. the program will raise this
    /// callback when reading values.
    pub fn stdin(&mut self, io_in: Box<FnMut() -> u8 + Send + 'static>) {
        self.io_in = io_in;
    }
    
    /// sets this programs stdout handler. the program will raise this
    /// callback when writing values.
    pub fn stdout(&mut self, io_out: Box<FnMut(u8) -> () + Send + 'static>) {
        self.io_out = io_out;
    }

    /// executes one instruction.    
    pub fn next(&mut self) -> Result<bool, Error> {
        if self.ins_ptr == self.ins.len() { return Ok(true); }
        match self.ins[self.ins_ptr] {
            '>' => { 
                if self.mem_ptr == (self.mem.len() - 1) { return Err(Error::MemPtrOutOfBounds(1)); }
                self.mem_ptr += 1; 
                self.ins_ptr += 1; },
            '<' => { 
                if self.mem_ptr == 0 { return Err(Error::MemPtrOutOfBounds(-1)); }
                self.mem_ptr -= 1; 
                self.ins_ptr += 1; },
            '+' => {
                if self.mem[self.mem_ptr] == 255 {
                    self.mem[self.mem_ptr] = 0;
                } else {
                    self.mem[self.mem_ptr] += 1;
                } self.ins_ptr += 1; 
            },
            '-' => { 
                if self.mem[self.mem_ptr] == 0 {
                    self.mem[self.mem_ptr] = 255;
                } else {
                    self.mem[self.mem_ptr] -= 1;
                } self.ins_ptr += 1; 
            },
            '.' => { 
                (self.io_out)(self.mem[self.mem_ptr]);   
                self.ins_ptr += 1; 
            },
            ',' => { 
                self.mem[self.mem_ptr] = (self.io_in)(); 
                self.ins_ptr += 1; 
            },
            '[' => {
                if self.mem[self.mem_ptr] == 0 {
                    let mut n = 0;
                    loop {
                        if self.ins_ptr == self.ins.len() { return Err(Error::InsPtrOutOfBounds(1)); }
                        if self.ins[self.ins_ptr] == '[' { n += 1; }
                        if self.ins[self.ins_ptr] == ']' { n -= 1; }
                        self.ins_ptr += 1;
                        if n == 0 { break; }
                    }
                } else { self.ins_ptr += 1; }
            },
            ']' => {
                if self.mem[self.mem_ptr] != 0 {
                    let mut n = 0;
                    loop {
                        if self.ins_ptr == 0 { return Err(Error::InsPtrOutOfBounds(-1)); }
                        if self.ins[self.ins_ptr] == ']' { n += 1; }
                        if self.ins[self.ins_ptr] == '[' { n -= 1; }
                        if n == 0 { self.ins_ptr += 1; break; }
                        self.ins_ptr -= 1;
                    }
                } else { 
                    self.ins_ptr += 1; 
                }
            },
            _ => { return Err(Error::InvalidInstruction); }
        };
        Ok(false)
    }
    /// runs the program.
    pub fn run(mut self) -> Result<bool, Error> {
        loop {
            let done = try!(self.next());
            if done { return Ok(true); }
        }
    }  
}