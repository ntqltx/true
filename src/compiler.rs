#[repr(u8)]
pub enum OpCode {
    Constant = 0,
    Add,
    Sub,
    Mul,
    Div,
    Negate,
    Return,
}

pub struct Compiler {
    code: Vec<u8>,
    constants: Vec<f64>
}