package main

import "core:mem"
import "core:fmt"
import "core:os"
import "core:strings"

import "vm"
foreign import compiler "target/release/libc.a"

FILE_EXT :: ".true"

CompileOutput :: struct {
    code: [^]u8,
    constants: [^]vm.Value,
    code_len: uint,
    constants_len: uint,
}

foreign compiler {
    compile :: proc(source_ptr: [^]u8, source_len: uint, is_repl: bool) -> ^CompileOutput ---
    free_compiled :: proc(output: ^CompileOutput) ---
}

main_interpret :: proc(source: string, is_repl: bool) -> vm.InterpretResult {
    output := compile(raw_data(source), uint(len(source)), is_repl)
    if output == nil {
        return .COMPILE_ERROR
    }
    defer free_compiled(output)

    chunk := vm.make_chunk(int(output.code_len), int(output.constants_len))
    defer vm.delete_chunk(chunk)

    append(&chunk.code, ..output.code[:output.code_len])
    append(&chunk.constants, ..output.constants[:output.constants_len])

    return vm.interpret(chunk)
}

repl :: proc() {
    buffer := make([]u8, 1024)
    defer delete(buffer)

    for {
        mem.zero_slice(buffer)
        fmt.print("> ")

        n, err := os.read(os.stdin, buffer)
        if n == 0 || err != nil {
            break
        }

        if result := main_interpret(cast(string) buffer[:n - 1], true);
        result != .OK {
            fmt.eprintln("error: interpret failed")
        }
    }

    fmt.println("")
}

run_file :: proc(path: string) {
    bytes, err := os.read_entire_file_from_path(path, context.allocator)
    if err != nil {
        fmt.eprintln("error: could not read file, check if path is correct")
        return
    }
    defer delete(bytes)

    if !strings.has_suffix(path, FILE_EXT) {
        fmt.eprintln("error: file isn't a True source file")
        return
    }
    main_interpret(cast(string) bytes, false)
}

main :: proc() {
    args := os.args

    my_data := vm.MyAllocatorData {}
    vm.my_allocator_data_init(&my_data)
    allocator := vm.my_allocator(&my_data)

    context.allocator = allocator

    vm.init_vm()
    defer vm.free_vm()

    switch len(args) {
        case 1: repl()
        case 2: run_file(args[1])
    }
}
