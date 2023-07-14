use chunk::Chunk;

use crate::chunk::OpCode;
use crate::debug::disassemble_chunk;
use crate::vm::VM;

pub mod chunk;
pub mod debug;
pub mod value;
pub mod vm;

fn main() {
    let mut my_chunk = Chunk::new();
    let constant: usize = my_chunk.add_constant(1.2);
    my_chunk.write_code(usize::from(OpCode::OpConstant), 123);
    my_chunk.write_code(constant, 123);
    my_chunk.write_code(usize::from(OpCode::OpNegate), 123);

    let a: usize = my_chunk.add_constant(2.0);
    my_chunk.write_code(usize::from(OpCode::OpConstant), 123);
    my_chunk.write_code(a, 123);

    let b: usize = my_chunk.add_constant(2.0);
    my_chunk.write_code(usize::from(OpCode::OpConstant), 123);
    my_chunk.write_code(b, 123);

    my_chunk.write_code(usize::from(OpCode::OpAdd), 123);

    my_chunk.write_code(usize::from(OpCode::OpReturn), 123);
    disassemble_chunk(&my_chunk, &String::from("test chunk"));

    let mut my_vm = VM::new();
    my_vm.interpret(my_chunk);
}
