# CocoaPic (Rust)

A PIC12F675 and PIC12C508 simulator with a native macOS window, rewritten in
Rust from the 2002–2003 Objective-C CocoaPic, which now lives in `legacy/`. Built the same way as
StorySearch: [gpui](https://crates.io/crates/gpui) 0.2 for the window, clap for
the CLI, a `bundle.sh` that makes an ad-hoc signed `.app`, and a `Makefile` of
the usual commands.

```bash
cargo run                         # the window, on the built-in blink demo
cargo run -- gui examples/blink.asm
cargo run -- --part 12c508        # the window, on the PIC12C508's demo
./bundle.sh && open CocoaPic.app  # the bundled app
cargo test                        # core tests plus headless window tests
make bench                        # full-speed MIPS on the example programs
```

The CLI runs the same core without the window:

```bash
cocoapic run examples/blink.asm --cycles 1000000   # run, print W/PC/GPIO/named RAM
cocoapic disasm program.hex                        # listing, with labels and notes
cocoapic asm examples/counter.asm                  # -> counter.hex + counter.cocoapic.json
cocoapic --part 12c508 run program.hex             # a hex file with no sidecar, as a 12C508
```

A source file names its part with `processor 12c508` or `list p=12c508`; a
saved hex file's sidecar records it. `--part` (12F675 by default) is for a
program that does not say.

## The window

- **Part** — the toolbar's PIC12F675 / PIC12C508 switch. Switching opens that
  part's demo, since a program for one core means nothing on the other.
- **Program** — all 1024 words (512 on the 12C508). Click the gutter dot for a breakpoint.
  Double-click a label, instruction or note to edit it; an instruction is typed
  as text (`movlw 0x20`, `btfss STATUS,Z`, `goto loop`) and assembled on Enter,
  Escape cancels. The PC row is tinted and followed while stepping.
- **Core** — W, PC (both editable), TMR0, watchdog progress, STATUS bits, the
  stack, cycle count and simulated time at 4 MHz.
- **Pins** — GP0–GP5 with direction and level. Click an input to drive it high
  or low. AN0–AN3 take a value 0–1023 for the A/D converter. On the 12C508,
  which has no A/D, OPTION and TRIS show here instead: they have no address.
- **RAM** — every SFR in both banks, then the 64 general-purpose bytes, in hex,
  decimal and binary. Double-click a value to poke it, or a GPR name to name it.
  Anything that changed since the machine last stopped is red, as in the old app.
- **Run / Halt / Step / Reset** in the toolbar and the Debug menu (⌘R, ⌘., F7 or
  ⌘', ⇧⌘R; ⌘B toggles a breakpoint). *Real time* paces a run at 1 µs per cycle so
  a blinking pin is visible; off, it runs as fast as it can (~190–280 MIPS on an
  M-series Mac).

Programs open from Intel HEX (MPASM/XC8 output, including the config word and
EEPROM) or from `.asm` source. Save writes `.hex` plus a
`<name>.cocoapic.json` sidecar with labels, register names, notes and
breakpoints, which opening the hex picks up again.

## What is simulated

**PIC12F675.** All 35 midrange instructions with their cycle counts; banked registers and their
mirrors; INDF/FSR; PCL/PCLATH computed jumps; the 8-level stack; TMR0 with
prescaler and T0CKI; the watchdog with postscaler (reset, or wake from SLEEP);
TMR1 on Fosc/4 with gating; interrupts from TMR0, INT, GPIO change, TMR1, A/D and
EEPROM; SLEEP; GPIO with TRISIO, weak pull-ups, and ANSEL/CMCON analog pins
reading 0; the data EEPROM with its 0x55/0xAA unlock; the A/D converter.

**PIC12C508.** All 33 baseline (12-bit) instructions, including OPTION and TRIS;
the start at 0x1FF, whose factory MOVLW leaves the oscillator calibration in W;
INDF/FSR; PCL computed jumps and CALL, which reach only the first 256 words; the
2-level stack, whose bottom falls off; TMR0 with prescaler and T0CKI, with no
overflow flag; the watchdog with postscaler; SLEEP, and every wake as a reset
that keeps RAM and sets GPWUF, /TO and /PD to say why; wake-on-change and weak
pull-ups on GP0, GP1 and GP3; OSCCAL. `examples/blink-12c508.asm` and
`examples/counter-12c508.asm` are the 12F675's two examples rewritten for it.

Not simulated: the comparator output and voltage reference, the external TMR1
oscillator, GP3 as /MCLR, the 12C508's wake-up delay, and the old app's PICkit USB read (`legacy/USB Pickit Interface.m`), which
needs the hardware to port and test.

## Layout

| file | what it does |
|---|---|
| `src/pic.rs` | the parts, the 12F675's register file and peripherals, `step` / `run`, and the instruction semantics both cores share |
| `src/pic/baseline.rs` | the 12C508's register file, peripherals, stack and resets |
| `src/isa.rs` | both instruction sets: decode/encode, one-line assembler and disassembler |
| `src/hex.rs` | Intel HEX load/save |
| `src/project.rs` | open/save with the sidecar, the source assembler, the demo |
| `src/main.rs` | CLI |
| `src/gui/` | the gpui window (`simulator.rs`), theme, actions, text input |

The old app generated one C function per opcode (16,384 of them, under
`legacy/PIC12F675/`); here the opcode is decoded and executed by one `match`, which also
fixes several bugs in the generated code (BTFSC tested with `|`, ADDWF/SUBWF with
some operands did nothing, RRF/RLF had their destinations swapped, CALL masked
the stack with 2 levels instead of 8).
