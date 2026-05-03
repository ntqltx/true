set quiet

set shell := ["bash", "-cu"]
set windows-shell := ["powershell.exe", "-c"]

alias t  := test
alias tc := test-nocapture
alias f  := fmt

# run compiler
[group("dev")]
[default, no-exit-message]
run: build-release
    odin run .

[group("dev")]
[unix, no-exit-message]
run-repl-binary:
    ./true

[private]
buildr:
    cargo build --release

# build release rust library
[group("dev")]
[unix]
build-release: buildr

[group("dev")]
[windows]
build-release: buildr
    rename target\release\c.lib libc.a

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