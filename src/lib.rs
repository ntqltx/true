use std::ffi::{CStr, c_char};
use std::ptr::{null_mut, slice_from_raw_parts_mut};

mod compiler;
mod expr;
mod parser;
mod scanner;
mod tokens;

use compiler::Compiler;
use expr::Statement;
use parser::Parser;
use scanner::Scanner;

#[repr(C)]
pub struct CompileOutput {
	code: *mut u8,
	constants: *mut f64,
	code_len: usize,
	constants_len: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn compile(source: *const c_char) -> *mut CompileOutput {
	let Ok(source) = (unsafe { CStr::from_ptr(source) }).to_str() else {
		return null_mut();
	};

	let tokens = match Scanner::new(source).scan_tokens() {
		Ok(t) => t,
		Err(e) => {
			eprintln!("{e}");
			return null_mut();
		}
	};

	let statements = match Parser::new(tokens).parse() {
		Ok(s) => s,
		Err(e) => {
			eprintln!("{e}");
			return null_mut();
		}
	};

	let mut compiler = Compiler::new();
	for Statement::Expression(expr) in &statements {
		compiler.compile_expr(expr);
	}

	let (code, constants) = compiler.finish();
	let (code_len, constants_len) = (code.len(), constants.len());

	Box::into_raw(Box::new(CompileOutput {
		code: Box::into_raw(code.into_boxed_slice()) as *mut u8,
		constants: Box::into_raw(constants.into_boxed_slice()) as *mut f64,
		code_len,
		constants_len,
	}))
}

#[unsafe(no_mangle)]
pub extern "C" fn free_compiled(output: *mut CompileOutput) {
	if output.is_null() {
		return;
	}

	unsafe {
		let output = Box::from_raw(output);
		drop(Box::from_raw(slice_from_raw_parts_mut(
			output.code,
			output.code_len,
		)));
		drop(Box::from_raw(slice_from_raw_parts_mut(
			output.constants,
			output.constants_len,
		)));
	}
}
