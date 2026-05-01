use std::ffi::c_char;

mod tokens;
mod scanner;

use scanner::Scanner;

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

    println!("{:?}", tokens);
    InterpretResult::OK
}