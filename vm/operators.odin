package vm

import "core:fmt"

OpCode :: enum u8 {
    OP_CONSTANT,
    OP_ADD, OP_SUB, OP_MUL, OP_DIV,
    OP_PRINT,
    OP_POP,
    OP_NEGATE,
    OP_RETURN,
}

binary_op :: #force_inline proc(op: proc(a, b: f64) -> f64) -> InterpretResult {
    b := pop(vm.stack)
    a := pop(vm.stack)

    if !is_number(a) || !is_number(b) {
        runtime_error("operands must be numbers")
        return .RUNTIME_ERROR
    }

    push(vm.stack, number_val(op(as_number(a), as_number(b))))
    return .OK
}

add :: proc(a, b: f64) -> f64 { return a + b }
sub :: proc(a, b: f64) -> f64 { return a - b }
mul :: proc(a, b: f64) -> f64 { return a * b }
div :: proc(a, b: f64) -> f64 { return a / b }

// test
// todo: error handling
runtime_error :: proc(msg: string) {
    fmt.eprintln("runtime error:", msg)
}