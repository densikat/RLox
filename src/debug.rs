use crate::Chunk;
use crate::chunk::OpCode;
use crate::value::Value;

pub fn print_value(value: Value) {
    print!("{value}")
}

fn constant_instruction(name: &String, chunk: &Chunk, offset: usize) -> usize {
    let constant: usize = chunk.code[offset + 1];
    let formatted = format!("{:<16} {:>4} '", name, constant);
    print!("{formatted}");
    print_value(chunk.constants[constant]);
    println!("'");
    return offset + 2;
}

fn simple_instruction(name: &String, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.lines[offset]);
    }

    let inst: OpCode = OpCode::from(chunk.code[offset]);
    return match inst {
        OpCode::OpAdd => { simple_instruction(&String::from("OP_ADD"), offset) }
        OpCode::OpConstant => { constant_instruction(&String::from("OP_CONSTANT"), chunk, offset) }
        OpCode::OpNegate => { simple_instruction(&String::from("OP_NEGATE"), offset) }
        OpCode::OpReturn => { simple_instruction(&String::from("OP_RETURN"), offset) }
        OpCode::OpUnknown => {
            print!("UNKNOWN VAL\n");
            offset + 1
        }
        _ => { 1 }
    };
}

pub fn disassemble_chunk(chunk: &Chunk, name: &String) {
    println!("=== {name} ===");

    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(&chunk, offset);
    }
}