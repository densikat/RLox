use crate::value::Value;

pub enum OpCode {
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
    OpUnknown,
}

impl From<OpCode> for usize {
    fn from(value: OpCode) -> usize {
        value as usize
    }
}

impl From<usize> for OpCode {
    fn from(value: usize) -> Self {
        match value {
            0 => OpCode::OpConstant,
            1 => OpCode::OpAdd,
            2 => OpCode::OpSubtract,
            3 => OpCode::OpMultiply,
            4 => OpCode::OpDivide,
            5 => OpCode::OpNegate,
            6 => OpCode::OpReturn,
            _ => OpCode::OpUnknown
        }
    }
}

pub struct Chunk {
    pub code: Vec<usize>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }
    pub fn write_code(&mut self, code: usize, line: usize) {
        self.code.push(code);
        self.lines.push(line);
    }
    pub fn get_code(&mut self, slot: usize) -> usize {
        self.code[slot]
    }
    pub fn add_constant(&mut self, constant: Value) -> usize {
        self.constants.push(constant);
        self.constants.len() - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_code_basic() {
        let mut my_chunk = Chunk::new();
        my_chunk.write_code(0);
        assert_eq!(my_chunk.get_code(0), 1);
    }
}