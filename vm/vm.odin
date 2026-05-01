package vm

import "core:fmt"
import "core:mem"

_DEBUG      :: false
DEBUG       :: _DEBUG
DEBUG_TRACE :: _DEBUG

VM :: struct {
    chunk: ^Chunk,
    ip: ^u8,
    stack: ^Stack,
}

vm : VM

init_vm :: proc() {
    vm = VM {
        chunk = nil,
        stack = make_stack(STACK_CAP),
    }
}

free_vm :: proc() {
    // free(vm.chunk)
}

InterpretResult :: enum i32 {
    OK,
    COMPILE_ERROR,
    RUNTIME_ERROR,
}

interpret :: proc(chunk: ^Chunk) -> InterpretResult {
    if len(chunk.code) == 0 {
        fmt.println("Empty chunk, nothing to execute")
        return .OK
    }

    vm.chunk = chunk
    vm.ip = &chunk.code[0]

    return run()
}

read_byte :: #force_inline proc() -> u8 {
    value := vm.ip^
    vm.ip = mem.ptr_offset(vm.ip, 1)

    return value
}

read_constant :: proc() -> Value {
    address := read_byte()
    value := vm.chunk.constants[address]

    return value
}

run :: proc() -> InterpretResult {
    for {
        when DEBUG_TRACE {
            fmt.println("--- STACK ---")
            fmt.print("[")
            
            for &value, i in &vm.stack.values {
                if &value == vm.stack.top {
                    break
                }
                if i > 0 {
                    fmt.print(", ")
                }
                fmt.print(value)
            }

            fmt.println("]")
        }

        instruction := cast(OpCode) read_byte()
        switch instruction {
            case .OP_CONSTANT:
                value := read_constant()
                push(vm.stack, value)

                when DEBUG {
                    fmt.println("Value:", value)
                }

            case .OP_ADD: apply(proc(a: Value, b: Value) -> Value { return a + b })
            case .OP_SUB: apply(proc(a: Value, b: Value) -> Value { return a - b })
            case .OP_MUL: apply(proc(a: Value, b: Value) -> Value { return a * b })
            case .OP_DIV: apply(proc(a: Value, b: Value) -> Value { return a / b })

            case .OP_NEGATE:
                value := pop(vm.stack)
                push(vm.stack, -value)

            case .OP_RETURN:
                value := pop(vm.stack)
                // fmt.println(value)

                return .OK
        }
    }

    // no return op
    return .RUNTIME_ERROR
}