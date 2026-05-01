set quiet

alias t  := test
alias tc := test-nocapture

# run compiler
[default]
[group("dev")]
[no-exit-message]
run: build-release
    odin run .

# build release rust library
[group("dev")]
build-release:
    cargo build --release
    cp target/release/libc.a .

# run vm
[group("dev")]
vm-run:
    odin run ./vm

# run all unit tests
[group("test")]
[no-exit-message]
test *ARGS:
    cargo test -- {{ARGS}}

# run all unit tests with print messages
[group("test")]
[no-exit-message]
test-nocapture: (test "--nocapture")