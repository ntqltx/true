set quiet

[default]
[no-exit-message]
run: build
    odin run .

build:
    cargo build --release
    cp target/release/libc.a .

vm-run:
    odin run ./vm