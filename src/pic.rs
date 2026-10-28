//! The PIC12F675: register file, peripherals and the instruction cycle.
//!
//! The Objective-C version generated one C function per 14-bit opcode — all
//! 16,384 of them — and dispatched through a table. Here an opcode is decoded
//! into an [`Op`] and executed by one `match`; the decode is a handful of
//! shifts and masks, so nothing is lost and the semantics live in one place
//! where they can be read against the datasheet (DS41190).
//!
//! Modelled: the full 35-instruction midrange set, both register banks with
//! their mirrors, INDF/FSR, PCL/PCLATH, the 8-level stack, TMR0 with its
//! prescaler, the watchdog with postscaler, TMR1, interrupts (TMR0, INT,
//! GPIO change, TMR1, A/D, EEPROM), SLEEP and wake, GPIO with TRISIO, weak
//! pull-ups and analog pin selection, the data EEPROM, and the A/D converter
//! fed from per-channel values the UI sets. Not modelled: the comparator's
//! output, the voltage reference, and instruction timing below one cycle.

use crate::isa::{Op, decode};

pub const ROM_WORDS: usize = 1024;
pub const EEPROM_BYTES: usize = 128;
pub const STACK_DEPTH: usize = 8;
/// The erased state of a program word: `ADDLW 0xFF`.
pub const ERASED: u16 = 0x3FFF;
/// Fosc/4 at the internal 4 MHz oscillator: one instruction cycle per µs.
pub const CYCLES_PER_SECOND: u64 = 1_000_000;
/// The watchdog's nominal period without postscaler, in instruction cycles.
const WDT_PERIOD: u64 = 18_000;
/// Instruction cycles an A/D conversion takes (11 TAD at Fosc/8, rounded).
const ADC_CYCLES: u32 = 22;

// Register addresses (bank-qualified, 0x00..=0xFF).
pub const INDF: u8 = 0x00;
pub const TMR0: u8 = 0x01;
pub const PCL: u8 = 0x02;
pub const STATUS: u8 = 0x03;
pub const FSR: u8 = 0x04;
pub const GPIO: u8 = 0x05;
pub const PCLATH: u8 = 0x0A;
pub const INTCON: u8 = 0x0B;
pub const PIR1: u8 = 0x0C;
pub const TMR1L: u8 = 0x0E;
pub const TMR1H: u8 = 0x0F;
pub const T1CON: u8 = 0x10;
pub const CMCON: u8 = 0x19;
pub const ADRESH: u8 = 0x1E;
pub const ADCON0: u8 = 0x1F;
pub const OPTION_REG: u8 = 0x81;
pub const TRISIO: u8 = 0x85;
pub const PIE1: u8 = 0x8C;
pub const PCON: u8 = 0x8E;
pub const OSCCAL: u8 = 0x90;
pub const WPU: u8 = 0x95;
pub const IOC: u8 = 0x96;
pub const VRCON: u8 = 0x99;
pub const EEDATA: u8 = 0x9A;
pub const EEADR: u8 = 0x9B;
pub const EECON1: u8 = 0x9C;
pub const EECON2: u8 = 0x9D;
pub const ADRESL: u8 = 0x9E;
pub const ANSEL: u8 = 0x9F;

// STATUS bits.
pub const C: u8 = 0;
pub const DC: u8 = 1;
pub const Z: u8 = 2;
pub const PD: u8 = 3;
pub const TO: u8 = 4;
pub const RP0: u8 = 5;

// INTCON bits.
const GPIF: u8 = 0;
const INTF: u8 = 1;
const T0IF: u8 = 2;
const GPIE: u8 = 3;
const INTE: u8 = 4;
const T0IE: u8 = 5;
const PEIE: u8 = 6;
const GIE: u8 = 7;

// PIR1 / PIE1 bits.
const TMR1IF: u8 = 0;
const ADIF: u8 = 6;
const EEIF: u8 = 7;

/// The configuration word at 0x2007.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config(pub u16);

impl Config {
    /// Internal oscillator, GP4/GP5 as I/O, watchdog off, MCLR as GP3 — what a
    /// bench program usually wants.
    pub const DEFAULT: Config = Config(0x3F94);

    pub fn wdt(self) -> bool {
        self.0 & 0x08 != 0
    }
}

/// The registers that exist in the part, by bank-qualified address. Anything
/// else in 0x00..=0x1F / 0x80..=0x9F is unimplemented and reads as zero.
pub const SFRS: &[(u8, &str)] = &[
    (INDF, "INDF"),
    (TMR0, "TMR0"),
    (PCL, "PCL"),
    (STATUS, "STATUS"),
    (FSR, "FSR"),
    (GPIO, "GPIO"),
    (PCLATH, "PCLATH"),
    (INTCON, "INTCON"),
    (PIR1, "PIR1"),
    (TMR1L, "TMR1L"),
    (TMR1H, "TMR1H"),
    (T1CON, "T1CON"),
    (CMCON, "CMCON"),
    (ADRESH, "ADRESH"),
    (ADCON0, "ADCON0"),
    (OPTION_REG, "OPTION_REG"),
    (TRISIO, "TRISIO"),
    (PIE1, "PIE1"),
    (PCON, "PCON"),
    (OSCCAL, "OSCCAL"),
    (WPU, "WPU"),
    (IOC, "IOC"),
    (VRCON, "VRCON"),
    (EEDATA, "EEDATA"),
    (EEADR, "EEADR"),
    (EECON1, "EECON1"),
    (EECON2, "EECON2"),
    (ADRESL, "ADRESL"),
    (ANSEL, "ANSEL"),
];

/// The name of a special function register at a bank-qualified address.
pub fn sfr_name(addr: u8) -> Option<&'static str> {
    let addr = canonical(addr).unwrap_or(addr);
    SFRS.iter().find(|(a, _)| *a == addr).map(|(_, n)| *n)
}

/// Where a bank-qualified address really lives: the registers mirrored in both
/// banks and the general-purpose RAM (0x20..=0x5F, mirrored at 0xA0..=0xDF)
/// fold onto their bank-0 address. `None` is an unimplemented location.
pub fn canonical(addr: u8) -> Option<u8> {
    match addr {
        0x00 | 0x80 => Some(INDF),
        0x02 | 0x82 => Some(PCL),
        0x03 | 0x83 => Some(STATUS),
        0x04 | 0x84 => Some(FSR),
        0x0A | 0x8A => Some(PCLATH),
        0x0B | 0x8B => Some(INTCON),
        0x20..=0x5F => Some(addr),
        0xA0..=0xDF => Some(addr - 0x80),
        a if SFRS.iter().any(|(s, _)| *s == a) => Some(a),
        _ => None,
    }
}

/// Why a run stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    /// It ran the cycles it was given.
    Budget,
    /// The PC reached a breakpoint.
    Breakpoint(u16),
    /// The caller asked it to.
    Halted,
}

/// One whole microcontroller.
#[derive(Clone)]
pub struct Pic {
    pub rom: Vec<u16>,
    pub config: Config,
    pub eeprom: [u8; EEPROM_BYTES],
    pub breakpoints: Vec<bool>,

    /// The register file, indexed by canonical address. Only the cells
    /// [`canonical`] can return are meaningful.
    regs: [u8; 256],
    pub w: u8,
    /// The 13-bit program counter. Fetches wrap at 1K words.
    pub pc: u16,
    pub stack: [u16; STACK_DEPTH],
    /// Index of the next free stack slot, wrapping: the hardware has no
    /// overflow flag, it simply overwrites.
    pub sp: usize,
    /// How many levels are live, for display; saturates at the depth.
    pub depth: usize,
    pub sleeping: bool,

    /// The level the outside world drives on each pin, GP0..GP5.
    pub pins_in: u8,
    /// Which pins have something driving them; an undriven input with its weak
    /// pull-up enabled reads high, otherwise low.
    pub pins_driven: u8,
    /// The analog value on AN0..AN3, 0..=1023.
    pub analog: [u16; 4],

    prescaler: u16,
    wdt: u64,
    tmr1_prescaler: u8,
    /// Cycles left on the A/D conversion in progress.
    adc_busy: u32,
    /// The EECON2 unlock sequence: 0 none, 1 saw 0x55, 2 saw 0xAA.
    ee_unlock: u8,
    /// GPIO as last read, for interrupt-on-change.
    gpio_latched: u8,
    /// The previous level of GP2, for the INT edge.
    int_level: bool,
    /// The previous level of GP2, for counting T0CKI edges.
    t0ck_level: bool,
    /// Cycles left during which a TMR0 write holds the timer.
    tmr0_hold: u8,

    pub cycles: u64,
}

impl Default for Pic {
    fn default() -> Self {
        Self::new()
    }
}

impl Pic {
    pub fn new() -> Self {
        let mut rom = vec![ERASED; ROM_WORDS];
        // The factory oscillator calibration: RETLW with the trim value.
        rom[ROM_WORDS - 1] = 0x3480;
        let mut pic = Pic {
            rom,
            config: Config::DEFAULT,
            eeprom: [0xFF; EEPROM_BYTES],
            breakpoints: vec![false; ROM_WORDS],
            regs: [0; 256],
            w: 0,
            pc: 0,
            stack: [0; STACK_DEPTH],
            sp: 0,
            depth: 0,
            sleeping: false,
            pins_in: 0,
            pins_driven: 0,
            analog: [0; 4],
            prescaler: 0,
            wdt: 0,
            tmr1_prescaler: 0,
            adc_busy: 0,
            ee_unlock: 0,
            gpio_latched: 0,
            int_level: false,
            t0ck_level: false,
            tmr0_hold: 0,
            cycles: 0,
        };
        pic.reset();
        pic
    }

    /// A power-on reset: registers to their datasheet POR values, the program,
    /// EEPROM, breakpoints and inputs left alone.
    pub fn reset(&mut self) {
        self.regs = [0; 256];
        self.regs[STATUS as usize] = 0x18;
        self.regs[OPTION_REG as usize] = 0xFF;
        self.regs[TRISIO as usize] = 0x3F;
        self.regs[OSCCAL as usize] = 0x80;
        self.regs[WPU as usize] = 0x37;
        self.regs[ANSEL as usize] = 0x0F;
        self.regs[PCON as usize] = 0x02;
        self.w = 0;
        self.pc = 0;
        self.stack = [0; STACK_DEPTH];
        self.sp = 0;
        self.depth = 0;
        self.sleeping = false;
        self.prescaler = 0;
        self.wdt = 0;
        self.tmr1_prescaler = 0;
        self.adc_busy = 0;
        self.ee_unlock = 0;
        self.int_level = self.pin_level(2);
        self.t0ck_level = self.int_level;
        self.gpio_latched = self.gpio_pins();
        self.tmr0_hold = 0;
        self.cycles = 0;
    }

    /// Clear the program to erased flash.
    pub fn erase(&mut self) {
        self.rom = vec![ERASED; ROM_WORDS];
        self.rom[ROM_WORDS - 1] = 0x3480;
        self.breakpoints = vec![false; ROM_WORDS];
    }

    // ---- register file -------------------------------------------------

    pub fn status(&self) -> u8 {
        self.regs[STATUS as usize]
    }

    pub fn flag(&self, bit: u8) -> bool {
        self.status() & (1 << bit) != 0
    }

    fn set_flag(&mut self, bit: u8, on: bool) {
        let s = &mut self.regs[STATUS as usize];
        if on {
            *s |= 1 << bit;
        } else {
            *s &= !(1 << bit);
        }
    }

    fn bank(&self) -> u8 {
        if self.flag(RP0) { 0x80 } else { 0 }
    }

    /// The bank-qualified address an instruction's 7-bit file operand names.
    pub fn direct(&self, f: u8) -> u8 {
        self.bank() | (f & 0x7F)
    }

    /// The level on one pin as the core sees it: the output latch if it is an
    /// output, the outside world (or its pull-up) if an input.
    pub fn pin_level(&self, pin: u8) -> bool {
        let bit = 1 << pin;
        let tris = self.regs[TRISIO as usize];
        // GP3 is input-only.
        if tris & bit == 0 && pin != 3 {
            return self.regs[GPIO as usize] & bit != 0;
        }
        if self.pins_driven & bit != 0 {
            return self.pins_in & bit != 0;
        }
        let pullups = self.regs[OPTION_REG as usize] & 0x80 == 0;
        pullups && self.regs[WPU as usize] & bit != 0
    }

    /// Which pins are configured as analog inputs and so read as 0.
    pub fn analog_pins(&self) -> u8 {
        let ansel = self.regs[ANSEL as usize];
        let mut mask = 0;
        for (an, gp) in [(0, 0), (1, 1), (2, 2), (3, 4)] {
            if ansel & (1 << an) != 0 {
                mask |= 1 << gp;
            }
        }
        // The comparator holds CIN+/CIN- (GP0/GP1) analog unless it is off.
        if self.regs[CMCON as usize] & 0x07 != 0x07 {
            mask |= 0b11;
        }
        mask
    }

    /// GPIO as a read sees it.
    fn gpio_pins(&self) -> u8 {
        let mut v = 0;
        for pin in 0..6 {
            if self.pin_level(pin) {
                v |= 1 << pin;
            }
        }
        v & !self.analog_pins() & 0x3F
    }

    /// Read a register by bank-qualified address, with the side effects a
    /// read has (GPIO latches the interrupt-on-change compare).
    pub fn read(&mut self, addr: u8) -> u8 {
        let v = self.peek(addr);
        if canonical(addr) == Some(GPIO) {
            self.gpio_latched = v;
        }
        v
    }

    /// Read a register without side effects — for display.
    pub fn peek(&self, addr: u8) -> u8 {
        let Some(a) = canonical(addr) else { return 0 };
        match a {
            INDF => {
                let fsr = self.regs[FSR as usize];
                if canonical(fsr) == Some(INDF) {
                    0
                } else {
                    self.peek(fsr)
                }
            }
            PCL => self.pc as u8,
            GPIO => self.gpio_pins(),
            EECON2 => 0,
            _ => self.regs[a as usize],
        }
    }

    /// Write a register by bank-qualified address, as an instruction would.
    pub fn write(&mut self, addr: u8, v: u8) {
        let Some(a) = canonical(addr) else { return };
        match a {
            INDF => {
                let fsr = self.regs[FSR as usize];
                if canonical(fsr) != Some(INDF) {
                    self.write(fsr, v);
                }
            }
            PCL => {
                self.pc = ((self.regs[PCLATH as usize] as u16 & 0x1F) << 8) | v as u16;
            }
            STATUS => {
                // /TO and /PD are read-only; IRP and RP1 are unimplemented.
                let keep = self.regs[a as usize] & 0x18;
                self.regs[a as usize] = (v & 0x27) | keep;
            }
            TMR0 => {
                self.regs[a as usize] = v;
                // A write clears an assigned prescaler and holds the count.
                if self.regs[OPTION_REG as usize] & 0x08 == 0 {
                    self.prescaler = 0;
                }
                self.tmr0_hold = 2;
            }
            GPIO => self.regs[a as usize] = v & 0x37,
            TRISIO => self.regs[a as usize] = v | 0x08,
            PCLATH => self.regs[a as usize] = v & 0x1F,
            ADCON0 => {
                let was = self.regs[a as usize];
                self.regs[a as usize] = v & 0xCF;
                // GO set with the converter on starts a conversion.
                if v & 0x02 != 0 && was & 0x02 == 0 && v & 0x01 != 0 {
                    self.adc_busy = ADC_CYCLES;
                }
            }
            EECON2 => {
                self.ee_unlock = match (self.ee_unlock, v) {
                    (_, 0x55) => 1,
                    (1, 0xAA) => 2,
                    _ => 0,
                };
            }
            EECON1 => self.eecon1(v),
            ANSEL => self.regs[a as usize] = v & 0x7F,
            PCON => self.regs[a as usize] = v & 0x03,
            WPU => self.regs[a as usize] = v & 0x37,
            IOC => self.regs[a as usize] = v & 0x3F,
            _ => self.regs[a as usize] = v,
        }
    }

    /// Poke a register from the UI. PCL and INDF behave as for an
    /// instruction; STATUS takes every bit, so /TO and /PD can be set too.
    pub fn poke(&mut self, addr: u8, v: u8) {
        match canonical(addr) {
            Some(STATUS) => self.regs[STATUS as usize] = v,
            _ => self.write(addr, v),
        }
    }

    fn eecon1(&mut self, v: u8) {
        let cur = self.regs[EECON1 as usize];
        // WR and RD can only be set by software, never cleared.
        let mut next = (v & 0x0C) | (cur & 0x03);
        let addr = self.regs[EEADR as usize] as usize % EEPROM_BYTES;
        if v & 0x01 != 0 {
            self.regs[EEDATA as usize] = self.eeprom[addr];
        }
        if v & 0x02 != 0 && v & 0x04 != 0 && self.ee_unlock == 2 {
            // A real write takes ~5 ms; it completes at once here.
            self.eeprom[addr] = self.regs[EEDATA as usize];
            self.regs[PIR1 as usize] |= 1 << EEIF;
            next &= !0x02;
        }
        self.ee_unlock = 0;
        self.regs[EECON1 as usize] = next & 0x0D;
    }

    // ---- stack ---------------------------------------------------------

    fn push(&mut self, addr: u16) {
        self.stack[self.sp] = addr & 0x1FFF;
        self.sp = (self.sp + 1) % STACK_DEPTH;
        self.depth = (self.depth + 1).min(STACK_DEPTH);
    }

    fn pop(&mut self) -> u16 {
        self.sp = (self.sp + STACK_DEPTH - 1) % STACK_DEPTH;
        self.depth = self.depth.saturating_sub(1);
        self.stack[self.sp]
    }

    /// The live stack, top first.
    pub fn stack_view(&self) -> Vec<u16> {
        (1..=self.depth)
            .map(|i| self.stack[(self.sp + STACK_DEPTH - i) % STACK_DEPTH])
            .collect()
    }

    // ---- execution -----------------------------------------------------

    /// The instruction the PC points at.
    pub fn current(&self) -> u16 {
        self.rom[self.pc as usize % ROM_WORDS]
    }

    /// Execute one instruction (or one cycle of sleep, or an interrupt
    /// vector), and return the instruction cycles it took.
    pub fn step(&mut self) -> u32 {
        if self.interrupt_pending() {
            if self.sleeping {
                self.sleeping = false;
                // Waking costs the instruction after SLEEP; the vector, if
                // GIE is set, comes on the next step.
                return self.tick(1);
            }
            if self.regs[INTCON as usize] & (1 << GIE) != 0 {
                self.regs[INTCON as usize] &= !(1 << GIE);
                self.push(self.pc);
                self.pc = 0x0004;
                return self.tick(2);
            }
        }
        if self.sleeping {
            return self.tick(1);
        }

        let word = self.current();
        self.pc = (self.pc + 1) & 0x1FFF;
        let cycles = self.exec(decode(word));
        self.tick(cycles)
    }

    /// Run until `budget` cycles have gone, a breakpoint is reached, or `stop`
    /// says so. A breakpoint at the PC on entry does not stop it: that is the
    /// one we are resuming from.
    pub fn run(&mut self, budget: u64, stop: &dyn Fn() -> bool) -> Stop {
        let end = self.cycles + budget;
        let mut steps = 0u32;
        while self.cycles < end {
            if steps > 0 && self.breakpoints[self.pc as usize % ROM_WORDS] {
                return Stop::Breakpoint(self.pc);
            }
            steps = steps.wrapping_add(1);
            self.step();
            if steps & 0xFFF == 0 && stop() {
                return Stop::Halted;
            }
        }
        if self.breakpoints[self.pc as usize % ROM_WORDS] {
            return Stop::Breakpoint(self.pc);
        }
        Stop::Budget
    }

    fn interrupt_pending(&self) -> bool {
        let intcon = self.regs[INTCON as usize];
        let on = |bit: u8| intcon & (1 << bit) != 0;
        let core = (on(T0IE) && on(T0IF)) || (on(INTE) && on(INTF)) || (on(GPIE) && on(GPIF));
        let peripheral =
            intcon & (1 << PEIE) != 0 && self.regs[PIR1 as usize] & self.regs[PIE1 as usize] != 0;
        core || peripheral
    }

    /// Execute a decoded instruction whose PC has already been advanced, and
    /// return its cycle count.
    fn exec(&mut self, op: Op) -> u32 {
        use Op::*;
        match op {
            Nop => 1,
            Movwf(f) => {
                let a = self.direct(f);
                self.write(a, self.w);
                self.pcl_cycles(a)
            }
            Clrw => {
                self.w = 0;
                self.set_flag(Z, true);
                1
            }
            Clrf(f) => {
                let a = self.direct(f);
                self.write(a, 0);
                self.set_flag(Z, true);
                self.pcl_cycles(a)
            }
            Subwf(f, d) => self.alu(f, d, |pic, x| {
                let w = pic.w;
                let r = x.wrapping_sub(w);
                pic.set_flag(C, x >= w);
                pic.set_flag(DC, (x & 0x0F) >= (w & 0x0F));
                pic.set_flag(Z, r == 0);
                r
            }),
            Addwf(f, d) => self.alu(f, d, |pic, x| {
                let w = pic.w;
                let (r, carry) = x.overflowing_add(w);
                pic.set_flag(C, carry);
                pic.set_flag(DC, (x & 0x0F) + (w & 0x0F) > 0x0F);
                pic.set_flag(Z, r == 0);
                r
            }),
            Decf(f, d) => self.alu(f, d, |pic, x| pic.zero(x.wrapping_sub(1))),
            Iorwf(f, d) => self.alu(f, d, |pic, x| pic.zero(x | pic.w)),
            Andwf(f, d) => self.alu(f, d, |pic, x| pic.zero(x & pic.w)),
            Xorwf(f, d) => self.alu(f, d, |pic, x| pic.zero(x ^ pic.w)),
            Movf(f, d) => self.alu(f, d, |pic, x| pic.zero(x)),
            Comf(f, d) => self.alu(f, d, |pic, x| pic.zero(!x)),
            Incf(f, d) => self.alu(f, d, |pic, x| pic.zero(x.wrapping_add(1))),
            Rrf(f, d) => self.alu(f, d, |pic, x| {
                let c = pic.flag(C) as u8;
                pic.set_flag(C, x & 1 != 0);
                (x >> 1) | (c << 7)
            }),
            Rlf(f, d) => self.alu(f, d, |pic, x| {
                let c = pic.flag(C) as u8;
                pic.set_flag(C, x & 0x80 != 0);
                (x << 1) | c
            }),
            Swapf(f, d) => self.alu(f, d, |_, x| x.rotate_left(4)),
            Decfsz(f, d) => {
                let mut zero = false;
                let c = self.alu(f, d, |_, x| {
                    let r = x.wrapping_sub(1);
                    zero = r == 0;
                    r
                });
                c + self.skip(zero)
            }
            Incfsz(f, d) => {
                let mut zero = false;
                let c = self.alu(f, d, |_, x| {
                    let r = x.wrapping_add(1);
                    zero = r == 0;
                    r
                });
                c + self.skip(zero)
            }
            Bcf(f, b) => {
                let a = self.direct(f);
                let v = self.read(a) & !(1 << b);
                self.write(a, v);
                self.pcl_cycles(a)
            }
            Bsf(f, b) => {
                let a = self.direct(f);
                let v = self.read(a) | (1 << b);
                self.write(a, v);
                self.pcl_cycles(a)
            }
            Btfsc(f, b) => {
                let a = self.direct(f);
                let clear = self.read(a) & (1 << b) == 0;
                1 + self.skip(clear)
            }
            Btfss(f, b) => {
                let a = self.direct(f);
                let set = self.read(a) & (1 << b) != 0;
                1 + self.skip(set)
            }
            Call(k) => {
                self.push(self.pc);
                self.pc = self.page(k);
                2
            }
            Goto(k) => {
                self.pc = self.page(k);
                2
            }
            Return => {
                self.pc = self.pop();
                2
            }
            Retfie => {
                self.pc = self.pop();
                self.regs[INTCON as usize] |= 1 << GIE;
                2
            }
            Retlw(k) => {
                self.w = k;
                self.pc = self.pop();
                2
            }
            Movlw(k) => {
                self.w = k;
                1
            }
            Iorlw(k) => {
                self.w = self.zero(self.w | k);
                1
            }
            Andlw(k) => {
                self.w = self.zero(self.w & k);
                1
            }
            Xorlw(k) => {
                self.w = self.zero(self.w ^ k);
                1
            }
            Sublw(k) => {
                let w = self.w;
                self.w = k.wrapping_sub(w);
                self.set_flag(C, k >= w);
                self.set_flag(DC, (k & 0x0F) >= (w & 0x0F));
                self.set_flag(Z, self.w == 0);
                1
            }
            Addlw(k) => {
                let w = self.w;
                let (r, carry) = w.overflowing_add(k);
                self.w = r;
                self.set_flag(C, carry);
                self.set_flag(DC, (w & 0x0F) + (k & 0x0F) > 0x0F);
                self.set_flag(Z, r == 0);
                1
            }
            Clrwdt => {
                self.clear_wdt();
                self.set_flag(TO, true);
                self.set_flag(PD, true);
                1
            }
            Sleep => {
                self.clear_wdt();
                self.set_flag(TO, true);
                self.set_flag(PD, false);
                self.sleeping = true;
                1
            }
        }
    }

    /// A byte-oriented file instruction: read f, compute, and store the result
    /// in W (`d` false) or back in f.
    fn alu(&mut self, f: u8, d: bool, op: impl FnOnce(&mut Self, u8) -> u8) -> u32 {
        let a = self.direct(f);
        let x = self.read(a);
        let r = op(self, x);
        if d {
            if canonical(a) == Some(STATUS) {
                // The flags the instruction set win over the written value.
                let flags = self.status() & 0x07;
                self.write(a, r);
                let s = &mut self.regs[STATUS as usize];
                *s = (*s & !0x07) | flags;
            } else {
                self.write(a, r);
            }
            self.pcl_cycles(a)
        } else {
            self.w = r;
            1
        }
    }

    fn zero(&mut self, r: u8) -> u8 {
        self.set_flag(Z, r == 0);
        r
    }

    /// A write to PCL is a jump and costs the second cycle.
    fn pcl_cycles(&self, a: u8) -> u32 {
        if canonical(a) == Some(PCL) { 2 } else { 1 }
    }

    fn skip(&mut self, take: bool) -> u32 {
        if take {
            self.pc = (self.pc + 1) & 0x1FFF;
            1
        } else {
            0
        }
    }

    /// GOTO/CALL take PCLATH<4:3> as the page bits above their 11-bit target.
    fn page(&self, k: u16) -> u16 {
        ((self.regs[PCLATH as usize] as u16 & 0x18) << 8) | (k & 0x7FF)
    }

    fn clear_wdt(&mut self) {
        self.wdt = 0;
        if self.regs[OPTION_REG as usize] & 0x08 != 0 {
            self.prescaler = 0;
        }
    }

    /// Advance the clocked peripherals by `n` instruction cycles.
    fn tick(&mut self, n: u32) -> u32 {
        for _ in 0..n {
            self.cycles += 1;
            self.tick_tmr0();
            self.tick_tmr1();
            self.tick_wdt();
            if self.adc_busy > 0 {
                self.adc_busy -= 1;
                if self.adc_busy == 0 {
                    self.finish_adc();
                }
            }
            self.watch_pins();
        }
        n
    }

    fn tick_tmr0(&mut self) {
        if self.sleeping {
            return;
        }
        let option = self.regs[OPTION_REG as usize];
        let edge = if option & 0x20 == 0 {
            true
        } else {
            // T0CKI on GP2: T0SE picks the edge.
            let level = self.pin_level(2);
            let was = self.t0ck_level;
            self.t0ck_level = level;
            if option & 0x10 == 0 {
                !was && level
            } else {
                was && !level
            }
        };
        if !edge {
            return;
        }
        if self.tmr0_hold > 0 {
            self.tmr0_hold -= 1;
            return;
        }
        if option & 0x08 == 0 {
            let rate = 2u16 << (option & 0x07);
            self.prescaler = self.prescaler.wrapping_add(1);
            if self.prescaler < rate {
                return;
            }
            self.prescaler = 0;
        }
        let t = &mut self.regs[TMR0 as usize];
        *t = t.wrapping_add(1);
        if *t == 0 {
            self.regs[INTCON as usize] |= 1 << T0IF;
        }
    }

    fn tick_tmr1(&mut self) {
        let t1con = self.regs[T1CON as usize];
        // On, and clocked from Fosc/4 (the external clock is not modelled).
        if t1con & 0x01 == 0 || t1con & 0x02 != 0 || self.sleeping {
            return;
        }
        // TMR1GE: count only while /T1G (GP4) is low.
        if t1con & 0x40 != 0 && self.pin_level(4) {
            return;
        }
        let rate = 1u8 << ((t1con >> 4) & 0x03);
        self.tmr1_prescaler = self.tmr1_prescaler.wrapping_add(1);
        if self.tmr1_prescaler < rate {
            return;
        }
        self.tmr1_prescaler = 0;
        let t = u16::from_le_bytes([self.regs[TMR1L as usize], self.regs[TMR1H as usize]])
            .wrapping_add(1);
        let [lo, hi] = t.to_le_bytes();
        self.regs[TMR1L as usize] = lo;
        self.regs[TMR1H as usize] = hi;
        if t == 0 {
            self.regs[PIR1 as usize] |= 1 << TMR1IF;
        }
    }

    fn tick_wdt(&mut self) {
        if !self.config.wdt() {
            return;
        }
        self.wdt += 1;
        let option = self.regs[OPTION_REG as usize];
        let post = if option & 0x08 != 0 {
            1u64 << (option & 0x07)
        } else {
            1
        };
        if self.wdt < WDT_PERIOD * post {
            return;
        }
        self.wdt = 0;
        if self.sleeping {
            // A timeout in sleep wakes the part where it slept.
            self.sleeping = false;
            self.set_flag(TO, false);
            self.set_flag(PD, false);
        } else {
            let cycles = self.cycles;
            self.reset();
            self.cycles = cycles;
            self.set_flag(TO, false);
        }
    }

    fn finish_adc(&mut self) {
        let adcon0 = self.regs[ADCON0 as usize];
        let ch = ((adcon0 >> 2) & 0x03) as usize;
        let v = self.analog[ch].min(1023);
        if adcon0 & 0x80 != 0 {
            // Right justified.
            self.regs[ADRESH as usize] = (v >> 8) as u8;
            self.regs[ADRESL as usize] = v as u8;
        } else {
            self.regs[ADRESH as usize] = (v >> 2) as u8;
            self.regs[ADRESL as usize] = (v << 6) as u8;
        }
        self.regs[ADCON0 as usize] &= !0x02;
        self.regs[PIR1 as usize] |= 1 << ADIF;
    }

    /// The pin-driven interrupt flags: the INT edge on GP2 and
    /// interrupt-on-change.
    fn watch_pins(&mut self) {
        let level = self.pin_level(2);
        if level != self.int_level {
            let rising = self.regs[OPTION_REG as usize] & 0x40 != 0;
            if level == rising {
                self.regs[INTCON as usize] |= 1 << INTF;
            }
            self.int_level = level;
        }
        let ioc = self.regs[IOC as usize];
        if ioc != 0 && (self.gpio_pins() ^ self.gpio_latched) & ioc != 0 {
            self.regs[INTCON as usize] |= 1 << GPIF;
        }
    }

    /// Drive a pin from outside, as a switch would.
    pub fn drive(&mut self, pin: u8, high: bool) {
        let bit = 1 << pin;
        self.pins_driven |= bit;
        if high {
            self.pins_in |= bit;
        } else {
            self.pins_in &= !bit;
        }
    }

    /// Whether a pin is an output.
    pub fn is_output(&self, pin: u8) -> bool {
        pin != 3 && self.regs[TRISIO as usize] & (1 << pin) == 0
    }

    /// The watchdog's progress toward a timeout, 0.0..1.0.
    pub fn wdt_fraction(&self) -> f32 {
        let option = self.regs[OPTION_REG as usize];
        let post = if option & 0x08 != 0 {
            1u64 << (option & 0x07)
        } else {
            1
        };
        self.wdt as f32 / (WDT_PERIOD * post) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::assemble;

    fn load(src: &[&str]) -> Pic {
        let mut pic = Pic::new();
        let names = crate::isa::Names::default();
        for (i, line) in src.iter().enumerate() {
            pic.rom[i] = assemble(line, &names).unwrap_or_else(|e| panic!("{line}: {e}"));
        }
        pic
    }

    #[test]
    fn add_sets_carry_digit_carry_and_zero() {
        let mut pic = load(&["movlw 0x88", "movwf 0x20", "movlw 0x78", "addwf 0x20,f"]);
        for _ in 0..4 {
            pic.step();
        }
        assert_eq!(pic.peek(0x20), 0x00);
        assert!(pic.flag(C) && pic.flag(DC) && pic.flag(Z));
    }

    #[test]
    fn subtract_is_f_minus_w_with_borrow_as_not_carry() {
        let mut pic = load(&["movlw 5", "movwf 0x20", "movlw 7", "subwf 0x20,w"]);
        for _ in 0..4 {
            pic.step();
        }
        assert_eq!(pic.w, 0xFE);
        assert!(!pic.flag(C));
        let mut pic = load(&["movlw 3", "sublw 10"]);
        pic.step();
        pic.step();
        assert_eq!(pic.w, 7);
        assert!(pic.flag(C));
    }

    #[test]
    fn call_return_and_retlw() {
        let mut pic = load(&["call 3", "nop", "goto 1", "retlw 0x42"]);
        assert_eq!(pic.step(), 2);
        assert_eq!(pic.pc, 3);
        assert_eq!(pic.stack_view(), vec![1]);
        pic.step();
        assert_eq!(pic.pc, 1);
        assert_eq!(pic.w, 0x42);
        assert!(pic.stack_view().is_empty());
    }

    #[test]
    fn bank_select_reaches_trisio_and_gpio_drives_pins() {
        let mut pic = load(&[
            "bsf STATUS,RP0",
            "movlw 0x3E",
            "movwf TRISIO",
            "bcf STATUS,RP0",
            "movlw 7",
            "movwf CMCON",
            "bsf GPIO,0",
        ]);
        for _ in 0..7 {
            pic.step();
        }
        assert_eq!(pic.peek(TRISIO), 0x3E);
        assert!(pic.is_output(0));
        assert!(pic.pin_level(0));
    }

    #[test]
    fn indirect_addressing_through_fsr() {
        let mut pic = load(&[
            "movlw 0x25",
            "movwf FSR",
            "movlw 9",
            "movwf INDF",
            "incf INDF,f",
        ]);
        for _ in 0..5 {
            pic.step();
        }
        assert_eq!(pic.peek(0x25), 10);
        // GPR is mirrored in bank 1.
        assert_eq!(pic.peek(0xA5), 10);
    }

    #[test]
    fn decfsz_loop_counts_cycles() {
        let mut pic = load(&["movlw 3", "movwf 0x20", "decfsz 0x20,f", "goto 2", "nop"]);
        while pic.pc != 4 {
            pic.step();
        }
        // movlw, movwf, 2×(decfsz 1 + goto 2), decfsz skipping (2).
        assert_eq!(pic.cycles, 1 + 1 + 2 * 3 + 2);
    }

    #[test]
    fn tmr0_overflow_interrupts() {
        let mut pic = load(&[
            "goto 5",
            "nop",
            "nop",
            "nop",
            "goto 4", // vector spins at 4
            "bsf STATUS,RP0",
            "movlw 0x88", // prescaler to WDT: TMR0 counts every cycle
            "movwf OPTION_REG",
            "bcf STATUS,RP0",
            "movlw 0xA0", // GIE | T0IE
            "movwf INTCON",
            "goto 11",
        ]);
        pic.run(400, &|| false);
        assert_eq!(pic.pc, 4);
        assert!(pic.peek(INTCON) & (1 << T0IF) != 0);
        assert!(pic.peek(INTCON) & (1 << GIE) == 0);
    }

    #[test]
    fn breakpoints_stop_a_run() {
        let mut pic = load(&["nop", "nop", "goto 0"]);
        pic.breakpoints[2] = true;
        assert_eq!(pic.run(1000, &|| false), Stop::Breakpoint(2));
        // Resuming from it does not stop at once.
        assert_eq!(pic.run(1000, &|| false), Stop::Breakpoint(2));
        assert!(pic.cycles > 4);
    }

    #[test]
    fn computed_goto_through_pcl() {
        let mut pic = load(&[
            "movlw 1",
            "call 3",
            "goto 2",
            "addwf PCL,f",
            "retlw 10",
            "retlw 20",
        ]);
        for _ in 0..4 {
            pic.step();
        }
        assert_eq!(pic.w, 20);
    }

    #[test]
    fn eeprom_write_needs_the_unlock_sequence() {
        let mut pic = load(&[
            "bsf STATUS,RP0",
            "movlw 4",
            "movwf EEADR",
            "movlw 0x5A",
            "movwf EEDATA",
            "bsf EECON1,2",
            "movlw 0x55",
            "movwf EECON2",
            "movlw 0xAA",
            "movwf EECON2",
            "bsf EECON1,1",
        ]);
        for _ in 0..11 {
            pic.step();
        }
        assert_eq!(pic.eeprom[4], 0x5A);
    }
}
