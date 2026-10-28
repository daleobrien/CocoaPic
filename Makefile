# Convenience wrapper around cargo, the cocoapic CLI and ./bundle.sh.
# Run `make` (or `make help`) for the list. FILE=... names a program for
# `make run-file`, `make disasm` and `make asm`.

.DEFAULT_GOAL := help
.PHONY: help check build debug test bench run gui app universal open demo disasm asm fmt clippy lint clean

FILE ?= examples/blink.asm

help:
	@echo "cocoapic — common tasks"
	@echo
	@echo "  make check      cargo check"
	@echo "  make build      cargo build --release"
	@echo "  make test       cargo test, including the headless window tests"
	@echo "  make bench      cargo bench — full-speed MIPS on the example programs"
	@echo "  make run        cargo run --release — the native window from source"
	@echo "  make app        ./bundle.sh — CocoaPic.app, arm64, ad-hoc signed"
	@echo "  make universal  ./bundle.sh --universal"
	@echo "  make open       open the bundled CocoaPic.app"
	@echo "  make demo       run FILE for 1M cycles in the terminal and print the state"
	@echo "  make disasm     print FILE's listing"
	@echo "  make asm        assemble FILE to .hex (+ .cocoapic.json)"
	@echo "  make lint       cargo fmt --check, then clippy"

check:
	cargo check --all-targets
build:
	cargo build --release
debug:
	cargo build
test:
	cargo test
bench:
	cargo bench --bench core
run:
	cargo run --release
app:
	./bundle.sh
universal:
	./bundle.sh --universal
open:
	open CocoaPic.app
demo:
	cargo run --release -- run "$(FILE)"
disasm:
	cargo run --release -- disasm "$(FILE)"
asm:
	cargo run --release -- asm "$(FILE)"
fmt:
	cargo fmt
clippy:
	cargo clippy --all-targets
lint:
	cargo fmt --check
	cargo clippy --all-targets
clean:
	cargo clean
