pub(crate) mod compiler;
pub(crate) mod value;

#[repr(u8)]
pub enum OpCode {
	Constant = 0,
	Add, Sub, Mul, Div,
	Print,
	Pop,
	Negate,
	Return,
}