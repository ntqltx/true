#![allow(dead_code)]
use crate::expr::Expr;
use crate::tokens::{LiteralValue::*, TokenType::*};

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
	constants: Vec<f64>,
}

impl Compiler {
	pub fn new() -> Self {
		Self {
			code: vec![],
			constants: vec![],
		}
	}

	pub fn compile_expr(&mut self, expr: &Expr) {
		match expr {
			Expr::Literal(value) => match value {
				NumberValue(n) => self.add_constant(*n),
				StringValue(_s) => todo!(),
				_ => todo!(),
			},
			Expr::Binary {
				left,
				operator,
				right,
			} => {
				self.compile_expr(left);
				self.compile_expr(right);

				match operator.token_type {
					Plus => self.add_op(OpCode::Add),
					Minus => self.add_op(OpCode::Sub),
					Star => self.add_op(OpCode::Mul),
					Slash => self.add_op(OpCode::Div),
					_ => todo!(),
				}
			}
			Expr::Unary { operator, right } => {
				self.compile_expr(right);

				match operator.token_type {
					Minus => self.add_op(OpCode::Negate),
					_ => todo!(),
				}
			}
			Expr::Grouping(expr) => self.compile_expr(expr),
		}
	}

	pub fn finish(mut self) -> (Vec<u8>, Vec<f64>) {
		self.add_op(OpCode::Return);
		(self.code, self.constants)
	}

	fn add_constant(&mut self, value: f64) {
		self.constants.push(value);

		let idx = self.previous_idx();
		self.add_op(OpCode::Constant);

		self.code.push(idx as u8)
	}

	fn add_op(&mut self, op: OpCode) {
		self.code.push(op as u8)
	}

	fn previous_idx(&self) -> usize {
		self.constants.len() - 1
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tokens::{LiteralValue, Token};

	#[test]
	fn number_literal_emits_constant() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Literal(NumberValue(42.0)));

		assert_eq!(c.constants, [42.0]);
		assert_eq!(c.code, [OpCode::Constant as u8, 0]);
	}

	#[test]
	fn two_constants_get_sequential_indices() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Literal(NumberValue(1.0)));
		c.compile_expr(&Expr::Literal(NumberValue(2.0)));

		assert_eq!(c.constants, [1.0, 2.0]);
		assert_eq!(
			c.code,
			[OpCode::Constant as u8, 0, OpCode::Constant as u8, 1]
		);
	}

	#[test]
	fn binary_add_emits_correct_bytes() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Binary {
			left: Box::new(Expr::Literal(NumberValue(1.0))),
			operator: Token {
				token_type: Plus,
				lexeme: "+".to_string(),
				literal: None,
				line_number: 1,
			},
			right: Box::new(Expr::Literal(NumberValue(2.0))),
		});

		assert_eq!(c.constants, [1.0, 2.0]);
		assert_eq!(
			c.code,
			[
				OpCode::Constant as u8,
				0,
				OpCode::Constant as u8,
				1,
				OpCode::Add as u8,
			]
		);
	}

	#[test]
	#[should_panic]
	fn non_number_literal_is_noop() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Literal(LiteralValue::True));

		assert!(c.code.is_empty());
		assert!(c.constants.is_empty());
	}

	#[test]
	fn grouping_emits_inner_expr() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Grouping(Box::new(Expr::Literal(NumberValue(7.0)))));

		assert_eq!(c.constants, [7.0]);
		assert_eq!(c.code, [OpCode::Constant as u8, 0]);
	}

	#[test]
	fn grouping_with_binary_emits_correct_bytes() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Grouping(Box::new(Expr::Binary {
			left: Box::new(Expr::Literal(NumberValue(1.0))),
			operator: Token {
				token_type: Star,
				lexeme: "*".to_string(),
				literal: None,
				line_number: 1,
			},
			right: Box::new(Expr::Literal(NumberValue(2.0))),
		})));

		assert_eq!(c.constants, [1.0, 2.0]);
		assert_eq!(
			c.code,
			[
				OpCode::Constant as u8,
				0,
				OpCode::Constant as u8,
				1,
				OpCode::Mul as u8,
			]
		);
	}

	#[test]
	#[should_panic]
	fn grouping_non_number_literal_panics() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Grouping(Box::new(Expr::Literal(LiteralValue::Nil))));
	}

	#[test]
	fn unary_negate_emits_correct_bytes() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Unary {
			operator: Token {
				token_type: Minus,
				lexeme: "-".to_string(),
				literal: None,
				line_number: 1,
			},
			right: Box::new(Expr::Literal(NumberValue(5.0))),
		});

		assert_eq!(c.constants, [5.0]);
		assert_eq!(c.code, [OpCode::Constant as u8, 0, OpCode::Negate as u8]);
	}

	#[test]
	fn double_negate_stacks_ops() {
		let mut c = Compiler::new();
		c.compile_expr(&Expr::Unary {
			operator: Token {
				token_type: Minus,
				lexeme: "-".to_string(),
				literal: None,
				line_number: 1,
			},
			right: Box::new(Expr::Unary {
				operator: Token {
					token_type: Minus,
					lexeme: "-".to_string(),
					literal: None,
					line_number: 1,
				},
				right: Box::new(Expr::Literal(NumberValue(3.0))),
			}),
		});

		assert_eq!(c.constants, [3.0]);
		assert_eq!(
			c.code,
			[
				OpCode::Constant as u8,
				0,
				OpCode::Negate as u8,
				OpCode::Negate as u8,
			]
		);
	}
}
