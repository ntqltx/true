set quiet

alias t  := test
alias tc := test-nocapture
alias f  := fmt

# run compiler
[group("dev")]
[default, no-exit-message]
run: build-release
    odin run .

# build release rust library
[unix]
[group("dev")]
build-release:
    cargo build --release
    cp target/release/libc.a .

[windows]
[group("dev")]
build-release:
    cargo build --release
    copy target\release\c.lib .

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

# format rust source files
[group("chore")]
fmt:
    cargo fmt