# CocoaPic (Rust)

A PIC12F675 simulator with a native macOS window, rewritten in Rust from the
2002–2003 Objective-C CocoaPic in the parent directory. Built the same way as
StorySearch: [gpui](https://crates.io/crates/gpui) 0.2 for the window, clap for
the CLI, a `bundle.sh` that makes an ad-hoc signed `.app`, and a `Makefile` of
the usual commands.

```bash
cargo run                         # the window, on the built-in blink demo
cargo run -- gui examples/blink.asm
./bundle.sh && open CocoaPic.app  # the bundled app
cargo test                        # core tests plus headless window tests
```

The CLI runs the same core without the window:

```bash
cocoapic run examples/blink.asm --cycles 1000000   # run, print W/PC/GPIO/named RAM
cocoapic disasm program.hex                        # listing, with labels and notes
cocoapic asm examples/counter.asm                  # -> counter.hex + counter.cocoapic.json
```

## The window

- **Program** — all 1024 words. Click the gutter dot for a breakpoint.
  Double-click a label, instruction or note to edit it; an instruction is typed
  as text (`movlw 0x20`, `btfss STATUS,Z`, `goto loop`) and assembled on Enter,
  Escape cancels. The PC row is tinted and followed while stepping.
- **Core** — W, PC (both editable), TMR0, watchdog progress, STATUS bits, the
  stack, cycle count and simulated time at 4 MHz.
- **Pins** — GP0–GP5 with direction and level. Click an input to drive it high
  or low. AN0–AN3 take a value 0–1023 for the A/D converter.
- **RAM** — every SFR in both banks, then the 64 general-purpose bytes, in hex,
  decimal and binary. Double-click a value to poke it, or a GPR name to name it.
  Anything that changed since the machine last stopped is red, as in the old app.
- **Run / Halt / Step / Reset** in the toolbar and the Debug menu (⌘R, ⌘., F7 or
  ⌘', ⇧⌘R; ⌘B toggles a breakpoint). *Real time* paces a run at 1 µs per cycle so
  a blinking pin is visible; off, it runs as fast as it can (~100–160 MIPS on an
  M-series Mac).

Programs open from Intel HEX (MPASM/XC8 output, including the config word and
EEPROM) or from `.asm` source. Save writes `.hex` plus a
`<name>.cocoapic.json` sidecar with labels, register names, notes and
breakpoints, which opening the hex picks up again.

## What is simulated

All 35 midrange instructions with their cycle counts; banked registers and their
mirrors; INDF/FSR; PCL/PCLATH computed jumps; the 8-level stack; TMR0 with
prescaler and T0CKI; the watchdog with postscaler (reset, or wake from SLEEP);
TMR1 on Fosc/4 with gating; interrupts from TMR0, INT, GPIO change, TMR1, A/D and
EEPROM; SLEEP; GPIO with TRISIO, weak pull-ups, and ANSEL/CMCON analog pins
reading 0; the data EEPROM with its 0x55/0xAA unlock; the A/D converter.

Not simulated: the comparator output and voltage reference, the external TMR1
oscillator, and the old app's PICkit USB read (`USB Pickit Interface.m`), which
needs the hardware to port and test.

## Layout

| file | what it does |
|---|---|
| `src/pic.rs` | the part: register file, peripherals, `step` / `run` |
| `src/isa.rs` | decode/encode, one-line assembler and disassembler |
| `src/hex.rs` | Intel HEX load/save |
| `src/project.rs` | open/save with the sidecar, the source assembler, the demo |
| `src/main.rs` | CLI |
| `src/gui/` | the gpui window (`simulator.rs`), theme, actions, text input |

The old app generated one C function per opcode (16,384 of them, under
`PIC12F675/`); here the opcode is decoded and executed by one `match`, which also
fixes several bugs in the generated code (BTFSC tested with `|`, ADDWF/SUBWF with
some operands did nothing, RRF/RLF had their destinations swapped, CALL masked
the stack with 2 levels instead of 8).
