# Development

[Back to the README](../README.md)

## Make targets

| Command | What it does |
| ------- | ------------ |
| `make build` | `cargo build --release` |
| `make run` | The window, from source |
| `make test` | The core tests, and the window tests, which draw the real window without a display |
| `make bench` | Full-speed MIPS on the example programs |
| `make lint` | `cargo fmt --check`, then clippy |
| `make fmt` | `cargo fmt` |
| `make app` | `CocoaPic.app`, arm64, ad-hoc signed |
| `make universal` | `CocoaPic.app` for arm64 and x86_64 |
| `make open` | Open the bundled app |
| `make demo FILE=…` | Run a program for a million cycles in the terminal and print the state |
| `make disasm FILE=…` | Print a program's listing |
| `make asm FILE=…` | Assemble a program to `.hex` and its `.cocoapic.json` sidecar |

`FILE` defaults to `examples/blink.asm`.

## The command line

```bash
cocoapic                                           # the window, on the demo
cocoapic gui examples/blink.asm                    # the window, on a program
cocoapic run examples/blink.asm --cycles 1000000   # run, print W/PC/GPIO/named RAM
cocoapic disasm program.hex                        # listing, with labels and notes
cocoapic asm examples/counter.asm                  # -> counter.hex + counter.cocoapic.json
cocoapic --part 12c508 run program.hex             # a hex file with no sidecar, as a 12C508
```

`--part` (12F675 by default) is only for a program that doesn't say which part
it is for.

## Benchmarks

`make bench` runs the core with *Real time* off on the example programs and
`benches/mixed.asm`. Criterion reports elements per second, one element per
instruction cycle, so `100 Melem/s` is 100 MIPS, or 100 times real time at
4 MHz.

## Source layout

| File | What it does |
| ---- | ------------ |
| `src/pic.rs` | The parts, the 12F675's register file and peripherals, `step` and `run`, and the instruction semantics both cores share |
| `src/pic/baseline.rs` | The 12C508's register file, peripherals, stack and resets |
| `src/isa.rs` | Both instruction sets: decode and encode, a one-line assembler and a disassembler |
| `src/hex.rs` | Intel HEX load and save |
| `src/project.rs` | Open and save with the sidecar, the source assembler, the demo |
| `src/main.rs` | The command line |
| `src/gui/` | The gpui window (`simulator.rs`), theme, actions and text input |

## The rewrite

CocoaPic began in 2002–2003 as an Objective-C app, which generated one C
function per opcode (16,384 of them). Here each opcode is decoded and executed
by one `match`, which also fixed several bugs in the generated code: BTFSC
tested with `|`, ADDWF and SUBWF did nothing with some operands, RRF and RLF had
their destinations swapped, and CALL masked the stack to 2 levels instead of 8.
The app's icon, `assets/AppIcon.icns`, is the original's.
