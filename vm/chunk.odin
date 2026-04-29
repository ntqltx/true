package vm

import "core:fmt"
import "core:strings"

OpCode :: enum u8 {
    OP_CONSTANT,
    OP_RETURN,
}

Chunk :: struct {
    code : [dynamic]u8,
    line_numbers : map[int]int,
    constants : ValueArray,
}

add_constant :: proc(chunk: ^Chunk, value: Value) {
    append(&chunk.constants, value)
    constant_idx := len(chunk.constants) - 1

    add_op(chunk, .OP_CONSTANT)
    append(&chunk.code, cast(u8) constant_idx)
}

add_op :: proc(chunk: ^Chunk, op: OpCode) {
    append(&chunk.code, cast(u8) op)
}

make_chunk :: proc(code_cap := 0, constants_cap := 0) -> ^Chunk {
    chunk := new(Chunk)
    code := make([dynamic]u8, 0, code_cap)
    constants := make(ValueArray, 0, constants_cap)

    chunk.code = code
    chunk.constants = constants

    return chunk
}

delete_chunk :: proc(chunk: ^Chunk) {
    delete(chunk.code)
    delete(chunk.constants)
    delete(chunk.line_numbers)

    free(chunk)
}

disassemble :: proc(chunk: ^Chunk) -> string {
    output : string = "Instructions:\n"
    
    index := 0
    inst_counter := 0

    for index < len(chunk.code) {
        byte_inst := chunk.code[index]
        op_code_str := OpCode(byte_inst)

        output = strings.concatenate({
            output, 
            fmt.tprintf("   %04v %v", inst_counter, op_code_str)
        })
        inst_counter += 1

        if op_code_str == .OP_CONSTANT {
            // Read the address (currently a single byte)
            // print address next to OpCode
            // advance pointer 2 bytes

            address := chunk.code[index + 1]
            output = strings.concatenate({output, fmt.tprintf(" %v", address)})
            index += 1
        }

        output = strings.concatenate({output, "\n"})
        index += 1
    }

    output = strings.concatenate({output, "Constants:\n"})
    for constant, index in chunk.constants {
        output = strings.concatenate({
            output, 
            fmt.tprintf("   %04v %v\n", index, constant)
        })
    }
    
    return output
}