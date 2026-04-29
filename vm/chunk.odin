package vm

OpCode :: enum {
    OP_RETURN,
}

Chunk :: struct {
    code : [dynamic]OpCode,
}