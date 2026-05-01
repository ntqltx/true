package main

import "core:mem"
import "core:fmt"
import "core:os"

import "vm"
foreign import comp "libc.a"

foreign comp {
    tokenize :: proc(source: string) -> vm.InterpretResult ---
}

compile_to_bytecode :: proc() -> ^vm.Chunk {
    chunk := vm.make_chunk()

    vm.add_constant(chunk, 3, 1)
    vm.add_op(chunk, .OP_RETURN, 2)

    return chunk
}

main_interpret :: proc(source: string) -> vm.InterpretResult {
    // execute
    chunk := compile_to_bytecode()
    defer vm.delete_chunk(chunk)
    
    tokenize(source)
    result := vm.interpret(chunk)
    return .OK
}

repl :: proc() {
    vm.init_vm()
    defer vm.free_vm()
    
    buffer := make([]u8, 1024)
    
    for {
        mem.zero_slice(buffer)
        fmt.print("> ")

        n_bytes_read, err := os.read(os.stdin, buffer)
        
        if n_bytes_read == 0 {
            break
        }

        input := cast(string) buffer[:n_bytes_read - 1]
        result := main_interpret(input)

        // fmt.println(result)

        if result != .OK {
            fmt.println("Failed to interpret")
            break
        }

        fmt.println("\nGot:", input)
    }

    fmt.println("")
    // fmt.println("result, ok:", result, ok)
    // fmt.println("buffer:", buffer[:20])
}

main :: proc() {
    args := os.args

    switch len(args) {
        case 1:
            // no input file given, start repl
            repl()
        case 2:
            // assume we were given a file, read it, compile and execute
            bytes, success := os.read_entire_file_from_path(args[1], context.allocator)
            file_contains := cast(string) bytes
            fmt.println(file_contains)

            // fmt.println("Got two args")
        case:
            fmt.println("Got something else")
    }
}