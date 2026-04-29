package vm

import "core:fmt"
import "core:mem"
import "core:os"

Allocator :: mem.Allocator
Allocator_Mode :: mem.Allocator_Mode
Allocator_Error :: mem.Allocator_Error

MyAllocatorData :: struct {
    // arena : []byte,
    allocator : Allocator,
}

my_allocator :: proc(my_allocator_data: ^MyAllocatorData) -> Allocator {
    return Allocator {
        procedure = my_allocator_proc,
        data = my_allocator_data,
    }
}

my_allocator_data_init :: proc(d: ^MyAllocatorData) {
    d.allocator = context.allocator
}

my_allocator_proc :: proc(
    allocator_data: rawptr, mode: Allocator_Mode, size, alignment: int,
    old_memory: rawptr, old_size: int, location := #caller_location

) -> ([]byte, Allocator_Error)
{
    allocator := (cast(^MyAllocatorData)allocator_data).allocator

    #partial switch mode {
        case .Alloc:
            fmt.println("ALLOC")
            bytes, error := mem.alloc_bytes(size, alignment, allocator, location)

            if error != nil {
                fmt.println("Failed to allocate")
                os.exit(1)
            }
            return bytes, nil

        case .Free:
            fmt.println("FREE")
            return nil, mem.free(cast(rawptr)old_memory, allocator, location)
    }

    fmt.println("Mode", mode, "not supported")
    os.exit(1) 
}

main :: proc () {
    my_data := MyAllocatorData {}
    my_allocator_data_init(&my_data)
    allocator := my_allocator(&my_data)

    context.allocator = allocator

    // code := make([dynamic]u8, 0, 0, allocator)
    code := make([dynamic]OpCode, 0, 2)
    defer delete(code)

    append(&code, OpCode.OP_RETURN)

    chunk : Chunk = {code}
    fmt.println(chunk)
}