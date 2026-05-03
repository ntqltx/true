#![allow(dead_code)]
use super::tokens::*;

#[derive(Debug)]
pub enum Expr {
	Literal(LiteralValue),
	Binary {
		left: Box<Expr>,
		operator: Token,
		right: Box<Expr>,
	},
	Unary {
		operator: Token,
		right: Box<Expr>,
	},
	Grouping(Box<Expr>),
}

#[derive(Debug)]
pub enum Statement {
	Expression(Expr),
}

impl std::fmt::Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Expr::Literal(v) => write!(f, "{v}"),
			Expr::Binary {
				left,
				operator,
				right,
			} => write!(f, "({} {left} {right})", operator.lexeme),
			Expr::Unary { operator, right } => {
				write!(f, "({} {right})", operator.lexeme)
			}
			Expr::Grouping(e) => write!(f, "(group {e})"),
		}
	}
}

impl std::fmt::Display for Statement {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Statement::Expression(e) => write!(f, "{e}"),
		}
	}
}
