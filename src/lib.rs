use std::ffi::c_char;

mod compiler;
mod tokens;
mod scanner;
mod parser;
mod expr;

use scanner::Scanner;
use parser::Parser;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum InterpretResult {
    OK,
    COMPILE_ERROR,
    RUNTIME_ERROR
}

#[unsafe(no_mangle)]
pub extern "C" fn tokenize(source: *const c_char) -> InterpretResult {
    let source = unsafe { std::ffi::CStr::from_ptr(source) };
    let source = match source.to_str() {
        Ok(s) => s,
        Err(_) => return InterpretResult::COMPILE_ERROR,
    };

    let scanner = Scanner::new(source);
    
    let tokens = match scanner.scan_tokens() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("{}", e);
            return InterpretResult::COMPILE_ERROR;
        }
    };

    let statements = match Parser::new(tokens).parse() {
        Ok(stmts) => stmts,
        Err(e) => {
            eprintln!("{e}");
            return InterpretResult::COMPILE_ERROR;
        }
    };

    for stmt in &statements {
        println!("{stmt}");
    }

    InterpretResult::OK
}