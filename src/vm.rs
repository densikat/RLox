use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::debug::print_value;
use crate::value::Value;
use crate::vm::InterpretResult::{InterpretCompileError, InterpretOk};

const STACK_MAX: usize = 256;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: [Value; STACK_MAX],
    pub stack_pointer: usize,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: [f64::default(); STACK_MAX],
            stack_pointer: 0,
        }
    }
    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }
    fn push(&mut self, value: Value) {
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;
    }
    fn pop(&mut self) -> Value {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }
    pub fn run(&mut self) -> InterpretResult {
        loop {
            let value: usize = self.chunk.code[self.ip]; // read instruction
            let op_code: OpCode = OpCode::from(value);
            self.ip = self.ip + 1; // advance ip
            match op_code {
                OpCode::OpAdd => {
                    let b: Value = self.pop();
                    let a: Value = self.pop();
                    self.push(a + b);
                    continue;
                }
                OpCode::OpNegate => {
                    let popped_val: Value = self.pop();
                    self.push(-popped_val);
                    continue;
                }
                OpCode::OpReturn => {
                    print_value(self.pop());
                    break InterpretOk;
                }
                OpCode::OpConstant => {
                    let const_index: usize = self.chunk.code[self.ip];
                    let constant: Value = self.chunk.constants[const_index]; // has been advanced
                    self.ip = self.ip + 1; // advance to next instruction
                    self.push(constant);
                    continue;
                }
                _ => {
                    println!("NO match");
                    InterpretCompileError
                }
            };
        }
    }
}

