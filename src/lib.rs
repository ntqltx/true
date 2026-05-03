mod ir;
mod lexer;

use ir::{
	compiler::Compiler,
	value::{Value, obj_free}
};
use lexer::{
	statements::Statement,
	parser::Parser,
	scanner::Scanner
};

use std::ptr::{null_mut, slice_from_raw_parts_mut};
use std::{slice, str};

#[repr(C)]
pub struct CompileOutput {
	code: *mut u8,
	constants: *mut Value,
	code_len: usize,
	constants_len: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn compile(source_ptr: *const u8, source_len: usize, is_repl: bool) -> *mut CompileOutput {
	if source_ptr.is_null() {
		return null_mut()
	}

	let Ok(source) = str::from_utf8(unsafe {
		slice::from_raw_parts(source_ptr, source_len)
	}) else {
		return null_mut()
	};

	let (code, constants) = match compile_source(source, is_repl) {
		Ok(result) => result,
		Err(e) => {
			eprintln!("{e}");
			return null_mut()
		}
	};
	let (code_len, constants_len) = (code.len(), constants.len());

	Box::into_raw(Box::new(CompileOutput {
		code: Box::into_raw(code.into_boxed_slice()) as *mut u8,
		constants: Box::into_raw(constants.into_boxed_slice()) as *mut Value,
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
		let consts = Box::from_raw(slice_from_raw_parts_mut(
			output.constants,
			output.constants_len,
		));

		for v in consts.iter() {
			if v.is_obj() {
				obj_free(v.as_obj());
			}
		}
		drop(consts);
		drop(Box::from_raw(slice_from_raw_parts_mut(
			output.code,
			output.code_len,
		)));
	}
}

fn compile_source(source: &str, is_repl: bool) -> Result<(Vec<u8>, Vec<Value>), String> {
    let tokens = Scanner::new(source).scan_tokens()?;
    let statements = Parser::new(tokens).parse()?;

    let mut compiler = Compiler::new();
	Statement::resolve(&statements, &mut compiler, is_repl);

    Ok(compiler.finish())
}