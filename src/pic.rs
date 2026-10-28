//! The parts: the PIC12F675 here, and the PIC12C508's register file and
//! peripherals in [`baseline`]. Both run the one [`Pic::exec`], compiled once
//! per core, so the 12F675's loop carries no test for which part it is.
//!
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

use serde::{Deserialize, Serialize};

use crate::isa::{Core, Op, decode};

pub mod baseline;

/// The parts the simulator models.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Part {
    /// The 14-bit midrange core: 1K words, 8-level stack, interrupts,
    /// peripherals.
    #[default]
    #[serde(rename = "PIC12F675")]
    Pic12F675,
    /// The 12-bit baseline core: 512 words, 2-level stack, no interrupts,
    /// TMR0 and GPIO only.
    #[serde(rename = "PIC12C508")]
    Pic12C508,
}

impl Part {
    pub const ALL: [Part; 2] = [Part::Pic12F675, Part::Pic12C508];

    pub fn name(self) -> &'static str {
        match self {
            Part::Pic12F675 => "PIC12F675",
            Part::Pic12C508 => "PIC12C508",
        }
    }

    /// A part by the name a source file or the command line gives it:
    /// `PIC12C508`, `p12c508` or `12c508`, in any case.
    pub fn from_name(name: &str) -> Option<Part> {
        let up = name.trim().to_ascii_uppercase();
        let bare = up
            .strip_prefix("PIC")
            .or_else(|| up.strip_prefix('P'))
            .unwrap_or(&up);
        Part::ALL
            .into_iter()
            .find(|p| p.name().strip_prefix("PIC") == Some(bare))
    }

    pub fn core(self) -> Core {
        match self {
            Part::Pic12F675 => Core::Midrange,
            Part::Pic12C508 => Core::Baseline,
        }
    }

    pub fn rom_words(self) -> usize {
        match self.core() {
            Core::Midrange => ROM_WORDS,
            Core::Baseline => baseline::ROM_WORDS,
        }
    }

    /// The erased state of a program word: all ones.
    pub fn erased(self) -> u16 {
        match self.core() {
            Core::Midrange => ERASED,
            Core::Baseline => baseline::ERASED,
        }
    }

    /// The last program word as it leaves the factory: the oscillator
    /// calibration, as a RETLW on the 12F675 and a MOVLW on the 12C508.
    fn calibration(self) -> u16 {
        match self.core() {
            Core::Midrange => 0x3480,
            Core::Baseline => baseline::CALIBRATION,
        }
    }

    /// Internal oscillator, watchdog off, MCLR off.
    pub fn default_config(self) -> Config {
        match self.core() {
            Core::Midrange => Config::DEFAULT,
            Core::Baseline => baseline::DEFAULT_CONFIG,
        }
    }

    /// The special function registers, by the address an instruction names
    /// (bank-qualified on the 12F675).
    pub fn sfrs(self) -> &'static [(u8, &'static str)] {
        match self.core() {
            Core::Midrange => SFRS,
            Core::Baseline => baseline::SFRS,
        }
    }

    pub fn sfr_name(self, addr: u8) -> Option<&'static str> {
        match self.core() {
            Core::Midrange => sfr_name(addr),
            Core::Baseline => baseline::SFRS
                .iter()
                .find(|(a, _)| *a == addr)
                .map(|(_, n)| *n),
        }
    }

    /// The general-purpose RAM.
    pub fn gprs(self) -> std::ops::RangeInclusive<u8> {
        match self.core() {
            Core::Midrange => 0x20..=0x5F,
            Core::Baseline => baseline::GPRS,
        }
    }

    /// STATUS bit names, bit 0 first; empty where a bit is unimplemented.
    pub fn status_bits(self) -> [&'static str; 8] {
        match self.core() {
            Core::Midrange => ["C", "DC", "Z", "PD", "TO", "RP0", "RP1", "IRP"],
            Core::Baseline => ["C", "DC", "Z", "PD", "TO", "PA0", "", "GPWUF"],
        }
    }
}

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
    match CANONICAL[addr as usize] {
        UNIMPLEMENTED => None,
        a => Some(a),
    }
}

/// [`canonical`] for every address, worked out at compile time: it is looked
/// up on every register access, so it is one load rather than a search.
static CANONICAL: [u8; 256] = {
    let mut table = [UNIMPLEMENTED; 256];
    let mut addr = 0;
    while addr < 256 {
        let a = addr as u8;
        table[addr] = match a {
            0x00 | 0x80 => INDF,
            0x02 | 0x82 => PCL,
            0x03 | 0x83 => STATUS,
            0x04 | 0x84 => FSR,
            0x0A | 0x8A => PCLATH,
            0x0B | 0x8B => INTCON,
            0x20..=0x5F => a,
            0xA0..=0xDF => a - 0x80,
            _ => {
                let mut i = 0;
                let mut found = UNIMPLEMENTED;
                while i < SFRS.len() {
                    if SFRS[i].0 == a {
                        found = a;
                    }
                    i += 1;
                }
                found
            }
        };
        addr += 1;
    }
    table
};

/// Which canonical registers a write simply stores to: the general-purpose
/// RAM and the SFRs [`Pic::write_special`] has no rule for. Most writes are
/// these, and one table load tells them from the rest.
static PLAIN_WRITE: [bool; 256] = {
    let special = [
        UNIMPLEMENTED,
        INDF,
        PCL,
        STATUS,
        TMR0,
        GPIO,
        TRISIO,
        OPTION_REG,
        CMCON,
        PCLATH,
        ADCON0,
        EECON2,
        EECON1,
        ANSEL,
        PCON,
        WPU,
        IOC,
    ];
    let mut table = [true; 256];
    let mut i = 0;
    while i < special.len() {
        table[special[i] as usize] = false;
        i += 1;
    }
    table
};

/// The [`CANONICAL`] entry for an address with no register. No real register
/// lives at 0xFF: that is bank 1's top GPR mirror, which is unimplemented.
const UNIMPLEMENTED: u8 = 0xFF;

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
    /// Which part this is. Set it through [`Pic::with_part`]: the program
    /// memory and register file are laid out for it.
    pub part: Part,
    pub rom: Vec<u16>,
    pub config: Config,
    pub eeprom: [u8; EEPROM_BYTES],
    pub breakpoints: Vec<bool>,

    /// The register file, indexed by canonical address. Only the cells
    /// [`canonical`] can return are meaningful (on the 12C508, see
    /// [`baseline`]).
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
    /// Whether [`Pic::watch_pins`] has seen the pins as they are, so it has
    /// nothing to do until something that sets their level changes: a write
    /// to a register behind them, a GPIO read, a reset, or the outside world
    /// between runs.
    pins_settled: bool,
    /// The previous level of GP2, for counting T0CKI edges.
    t0ck_level: bool,
    /// Cycles left during which a TMR0 write holds the timer.
    tmr0_hold: u8,

    /// Each program word as last decoded, with the word it came from. `rom` is
    /// public and can change under it, so a fetch checks the word still
    /// matches; that costs a compare, where decoding costs a dozen branches.
    decoded: Box<[(u16, Op); ROM_WORDS]>,

    pub cycles: u64,
}

impl Default for Pic {
    fn default() -> Self {
        Self::new()
    }
}

impl Pic {
    /// A PIC12F675.
    pub fn new() -> Self {
        Self::with_part(Part::Pic12F675)
    }

    /// A part fresh from the factory: erased but for the oscillator
    /// calibration, and just powered on.
    pub fn with_part(part: Part) -> Self {
        let mut pic = Pic {
            part,
            rom: Vec::new(),
            config: part.default_config(),
            eeprom: [0xFF; EEPROM_BYTES],
            breakpoints: Vec::new(),
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
            pins_settled: false,
            t0ck_level: false,
            tmr0_hold: 0,
            // A word no program word is (they are at most 14 bits), so the
            // first fetch at each address decodes.
            decoded: Box::new([(u16::MAX, Op::Nop); ROM_WORDS]),
            cycles: 0,
        };
        pic.erase();
        pic.reset();
        pic
    }

    /// A power-on reset: registers to their datasheet POR values, the program,
    /// EEPROM, breakpoints and inputs left alone.
    pub fn reset(&mut self) {
        match self.part.core() {
            Core::Midrange => self.reset14(),
            Core::Baseline => self.reset12(baseline::Cause::PowerOn),
        }
    }

    fn reset14(&mut self) {
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
        self.int_level = self.level14(2);
        self.t0ck_level = self.int_level;
        self.gpio_latched = self.gpio_pins();
        self.pins_settled = false;
        self.tmr0_hold = 0;
        self.cycles = 0;
    }

    /// Clear the program to erased flash, all but the factory calibration in
    /// the last word.
    pub fn erase(&mut self) {
        let words = self.part.rom_words();
        self.rom = vec![self.part.erased(); words];
        self.rom[words - 1] = self.part.calibration();
        self.breakpoints = vec![false; words];
    }

    /// Whether the configuration word turns the watchdog on.
    pub fn wdt_enabled(&self) -> bool {
        match self.part.core() {
            Core::Midrange => self.config.0 & 0x08 != 0,
            Core::Baseline => self.config.0 & 0x04 != 0,
        }
    }

    /// OPTION as last written. On the 12C508 only the OPTION instruction can
    /// write it, and nothing can read it back.
    pub fn option(&self) -> u8 {
        self.regs[OPTION_REG as usize]
    }

    /// The GPIO direction latch: TRISIO, or on the 12C508 what the TRIS
    /// instruction last wrote.
    pub fn tris(&self) -> u8 {
        self.regs[TRISIO as usize]
    }

    // ---- register file -------------------------------------------------

    pub fn status(&self) -> u8 {
        self.regs[STATUS as usize]
    }

    pub fn flag(&self, bit: u8) -> bool {
        self.status() & (1 << bit) != 0
    }

    #[inline(always)]
    fn set_flag(&mut self, bit: u8, on: bool) {
        // Without a branch: the ALU's flags are as good as random.
        let s = &mut self.regs[STATUS as usize];
        *s = (*s & !(1 << bit)) | ((on as u8) << bit);
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
        match self.part.core() {
            Core::Midrange => self.level14(pin),
            Core::Baseline => self.level12(pin),
        }
    }

    fn level14(&self, pin: u8) -> bool {
        self.levels14() & (1 << pin) != 0
    }

    /// [`Pic::level14`] for all six pins at once, GP0 in bit 0: the output
    /// latch on outputs, else what drives the pin, else its pull-up.
    #[inline]
    fn levels14(&self) -> u8 {
        // GP3 is input-only.
        let inputs = self.regs[TRISIO as usize] | 0x08;
        let pullups = if self.regs[OPTION_REG as usize] & 0x80 == 0 {
            self.regs[WPU as usize]
        } else {
            0
        };
        let driven = self.pins_driven & inputs;
        let floating = !self.pins_driven & inputs;
        ((self.regs[GPIO as usize] & !inputs) | (self.pins_in & driven) | (pullups & floating))
            & 0x3F
    }

    /// Which pins are configured as analog inputs and so read as 0.
    pub fn analog_pins(&self) -> u8 {
        match self.part.core() {
            Core::Midrange => self.analog14(),
            Core::Baseline => 0,
        }
    }

    #[inline]
    fn analog14(&self) -> u8 {
        // AN0..AN2 are GP0..GP2; AN3 is GP4.
        let ansel = self.regs[ANSEL as usize];
        let mut mask = (ansel & 0x07) | ((ansel & 0x08) << 1);
        // The comparator holds CIN+/CIN- (GP0/GP1) analog unless it is off.
        if self.regs[CMCON as usize] & 0x07 != 0x07 {
            mask |= 0b11;
        }
        mask
    }

    /// GPIO as a read sees it.
    #[inline]
    fn gpio_pins(&self) -> u8 {
        self.levels14() & !self.analog14()
    }

    /// Read a register by canonical address (or [`UNIMPLEMENTED`]), with the
    /// side effects a read has (GPIO latches the interrupt-on-change compare).
    #[inline(always)]
    fn read_reg(&mut self, a: u8) -> u8 {
        let v = self.peek_reg(a);
        if a == GPIO {
            self.gpio_latched = v;
            self.pins_settled = false;
        }
        v
    }

    /// Read a register without side effects — for display. The address is
    /// bank-qualified on the 12F675 and 0x00..=0x1F on the 12C508.
    pub fn peek(&self, addr: u8) -> u8 {
        match self.part.core() {
            Core::Midrange => self.peek14(addr),
            Core::Baseline => self.peek12(addr),
        }
    }

    fn peek14(&self, addr: u8) -> u8 {
        self.peek_reg(CANONICAL[addr as usize])
    }

    /// [`Pic::peek14`] by canonical address, or [`UNIMPLEMENTED`]. INDF goes
    /// through FSR, which can name any register but INDF itself.
    #[inline(always)]
    fn peek_reg(&self, a: u8) -> u8 {
        if a == INDF {
            match CANONICAL[self.regs[FSR as usize] as usize] {
                INDF => 0,
                r => self.peek_direct(r),
            }
        } else {
            self.peek_direct(a)
        }
    }

    #[inline(always)]
    fn peek_direct(&self, a: u8) -> u8 {
        match a {
            PCL => self.pc as u8,
            GPIO => self.gpio_pins(),
            EECON2 | UNIMPLEMENTED => 0,
            _ => self.regs[a as usize],
        }
    }

    /// Write a register by bank-qualified address, as an instruction would.
    fn write14(&mut self, addr: u8, v: u8) {
        self.write_reg(CANONICAL[addr as usize], v)
    }

    /// [`Pic::write14`] by canonical address, or [`UNIMPLEMENTED`].
    #[inline(always)]
    fn write_reg(&mut self, a: u8, v: u8) {
        if a == INDF {
            match CANONICAL[self.regs[FSR as usize] as usize] {
                INDF => {}
                r => self.write_direct(r, v),
            }
        } else {
            self.write_direct(a, v)
        }
    }

    #[inline(always)]
    fn write_direct(&mut self, a: u8, v: u8) {
        if PLAIN_WRITE[a as usize] {
            self.regs[a as usize] = v;
            return;
        }
        self.write_special(a, v)
    }

    /// A write to a register that does more than store the value.
    fn write_special(&mut self, a: u8, v: u8) {
        match a {
            UNIMPLEMENTED | INDF => {}
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
            GPIO => {
                self.regs[a as usize] = v & 0x37;
                self.pins_settled = false;
            }
            TRISIO => {
                self.regs[a as usize] = v | 0x08;
                self.pins_settled = false;
            }
            OPTION_REG | CMCON => {
                self.regs[a as usize] = v;
                self.pins_settled = false;
            }
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
            ANSEL => {
                self.regs[a as usize] = v & 0x7F;
                self.pins_settled = false;
            }
            PCON => self.regs[a as usize] = v & 0x03,
            WPU => {
                self.regs[a as usize] = v & 0x37;
                self.pins_settled = false;
            }
            IOC => {
                self.regs[a as usize] = v & 0x3F;
                self.pins_settled = false;
            }
            _ => self.regs[a as usize] = v,
        }
    }

    /// Poke a register from the UI. PCL and INDF behave as for an
    /// instruction; STATUS takes every bit, so /TO and /PD can be set too.
    pub fn poke(&mut self, addr: u8, v: u8) {
        match self.part.core() {
            Core::Midrange => match canonical(addr) {
                Some(STATUS) => self.regs[STATUS as usize] = v,
                _ => self.write14(addr, v),
            },
            Core::Baseline => match addr {
                STATUS => self.regs[STATUS as usize] = v,
                _ => self.write12(addr, v),
            },
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

    #[inline(always)]
    fn push(&mut self, addr: u16) {
        self.stack[self.sp] = addr & 0x1FFF;
        self.sp = (self.sp + 1) % STACK_DEPTH;
        self.depth = (self.depth + 1).min(STACK_DEPTH);
    }

    #[inline(always)]
    fn pop(&mut self) -> u16 {
        self.sp = (self.sp + STACK_DEPTH - 1) % STACK_DEPTH;
        self.depth = self.depth.saturating_sub(1);
        self.stack[self.sp]
    }

    /// The live stack, top first.
    pub fn stack_view(&self) -> Vec<u16> {
        if self.part.core() == Core::Baseline {
            return self.stack[..self.depth].to_vec();
        }
        (1..=self.depth)
            .map(|i| self.stack[(self.sp + STACK_DEPTH - i) % STACK_DEPTH])
            .collect()
    }

    // ---- execution -----------------------------------------------------

    /// The instruction the PC points at.
    pub fn current(&self) -> u16 {
        self.rom[self.pc as usize % self.rom.len()]
    }

    /// Execute one instruction (or one cycle of sleep, or an interrupt
    /// vector), and return the instruction cycles it took.
    pub fn step(&mut self) -> u32 {
        // The pins may have changed from outside since the last step.
        self.pins_settled = false;
        match self.part.core() {
            Core::Midrange => self.step14(),
            Core::Baseline => self.step12(),
        }
    }

    #[inline(always)]
    fn step14(&mut self) -> u32 {
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

        let op = self.fetch::<ROM_WORDS>(decode);
        self.pc = (self.pc + 1) & 0x1FFF;
        let cycles = self.exec::<false>(op);
        self.tick(cycles)
    }

    /// The instruction at the PC, decoded with `decode` unless the word is the
    /// one decoded there last time.
    #[inline(always)]
    fn fetch<const WORDS: usize>(&mut self, decode: fn(u16) -> Op) -> Op {
        let at = self.pc as usize % WORDS;
        let word = self.rom[at];
        let cached = &mut self.decoded[at];
        if cached.0 != word {
            *cached = (word, decode(word));
        }
        cached.1
    }

    /// Run until `budget` cycles have gone, a breakpoint is reached, or `stop`
    /// says so. A breakpoint at the PC on entry does not stop it: that is the
    /// one we are resuming from.
    pub fn run(&mut self, budget: u64, stop: impl Fn() -> bool) -> Stop {
        self.pins_settled = false;
        // The part is chosen once here, so each core's loop is its own code.
        match self.part.core() {
            Core::Midrange => self.run_with::<ROM_WORDS>(budget, stop, Self::step14, Self::doze14),
            Core::Baseline => {
                self.run_with::<{ baseline::ROM_WORDS }>(budget, stop, Self::step12, Self::doze12)
            }
        }
    }

    /// The run loop, given the core's `step` and `doze`.
    #[inline(always)]
    fn run_with<const WORDS: usize>(
        &mut self,
        budget: u64,
        stop: impl Fn() -> bool,
        step: impl Fn(&mut Self) -> u32,
        doze: impl Fn(&mut Self, u64) -> bool,
    ) -> Stop {
        let end = self.cycles + budget;
        let mut steps = 0u32;
        // Breakpoints only change between runs; most runs have none to check.
        let any_breakpoints = self.breakpoints.contains(&true);
        while self.cycles < end {
            if steps > 0 {
                if any_breakpoints && self.breakpoints[self.pc as usize % WORDS] {
                    return Stop::Breakpoint(self.pc);
                }
                // A step has run since the pins last changed from outside,
                // so asleep the part can skip ahead to whatever wakes it.
                if self.sleeping && doze(self, end - self.cycles) {
                    continue;
                }
            }
            steps = steps.wrapping_add(1);
            step(self);
            if steps & 0xFFF == 0 && stop() {
                return Stop::Halted;
            }
        }
        if self.breakpoints[self.pc as usize % WORDS] {
            return Stop::Breakpoint(self.pc);
        }
        Stop::Budget
    }

    /// Asleep, pass up to `room` cycles at once, stopping short of anything
    /// that could wake the part: the watchdog timing out, or the A/D finishing
    /// (its flag can be an interrupt). Taken one by one those cycles would
    /// each find no interrupt and tick once; a pin change or interrupt flag
    /// that wakes it was already seen by the step before. Returns whether it
    /// passed any.
    fn doze14(&mut self, room: u64) -> bool {
        if self.interrupt_pending() {
            return false;
        }
        let mut cycles = room.min(u32::MAX as u64);
        if self.adc_busy > 0 {
            cycles = cycles.min(self.adc_busy as u64);
        }
        if self.config.0 & 0x08 != 0 {
            cycles = cycles.min(self.wdt_limit().saturating_sub(self.wdt + 1));
        }
        if cycles == 0 {
            return false;
        }
        self.tick(cycles as u32);
        true
    }

    #[inline(always)]
    fn interrupt_pending(&self) -> bool {
        let intcon = self.regs[INTCON as usize];
        // Each core source's enable sits three bits above its flag.
        const _: () = assert!(T0IE == T0IF + 3 && INTE == INTF + 3 && GPIE == GPIF + 3);
        let core = intcon & (intcon >> 3) & 0x07 != 0;
        let peripheral =
            intcon & (1 << PEIE) != 0 && self.regs[PIR1 as usize] & self.regs[PIE1 as usize] != 0;
        core || peripheral
    }

    /// Execute a decoded instruction whose PC has already been advanced, and
    /// return its cycle count. `BASE` picks the core: false for the 12F675's
    /// midrange, true for the 12C508's baseline. It is a constant, so each
    /// core gets its own copy of this with the other's paths compiled out.
    #[inline(always)]
    fn exec<const BASE: bool>(&mut self, op: Op) -> u32 {
        use Op::*;
        match op {
            Nop => 1,
            Movwf(f) => {
                let a = self.file::<BASE>(f);
                self.wr::<BASE>(a, self.w);
                self.pcl_cycles::<BASE>(a)
            }
            Clrw => {
                self.w = 0;
                self.set_flag(Z, true);
                1
            }
            Clrf(f) => {
                let a = self.file::<BASE>(f);
                self.wr::<BASE>(a, 0);
                self.set_flag(Z, true);
                self.pcl_cycles::<BASE>(a)
            }
            Subwf(f, d) => self.alu::<BASE>(f, d, |pic, x| {
                let w = pic.w;
                let r = x.wrapping_sub(w);
                pic.set_flag(C, x >= w);
                pic.set_flag(DC, (x & 0x0F) >= (w & 0x0F));
                pic.set_flag(Z, r == 0);
                r
            }),
            Addwf(f, d) => self.alu::<BASE>(f, d, |pic, x| {
                let w = pic.w;
                let (r, carry) = x.overflowing_add(w);
                pic.set_flag(C, carry);
                pic.set_flag(DC, (x & 0x0F) + (w & 0x0F) > 0x0F);
                pic.set_flag(Z, r == 0);
                r
            }),
            Decf(f, d) => self.alu::<BASE>(f, d, |pic, x| pic.zero(x.wrapping_sub(1))),
            Iorwf(f, d) => self.alu::<BASE>(f, d, |pic, x| pic.zero(x | pic.w)),
            Andwf(f, d) => self.alu::<BASE>(f, d, |pic, x| pic.zero(x & pic.w)),
            Xorwf(f, d) => self.alu::<BASE>(f, d, |pic, x| pic.zero(x ^ pic.w)),
            Movf(f, d) => self.alu::<BASE>(f, d, |pic, x| pic.zero(x)),
            Comf(f, d) => self.alu::<BASE>(f, d, |pic, x| pic.zero(!x)),
            Incf(f, d) => self.alu::<BASE>(f, d, |pic, x| pic.zero(x.wrapping_add(1))),
            Rrf(f, d) => self.alu::<BASE>(f, d, |pic, x| {
                let c = pic.flag(C) as u8;
                pic.set_flag(C, x & 1 != 0);
                (x >> 1) | (c << 7)
            }),
            Rlf(f, d) => self.alu::<BASE>(f, d, |pic, x| {
                let c = pic.flag(C) as u8;
                pic.set_flag(C, x & 0x80 != 0);
                (x << 1) | c
            }),
            Swapf(f, d) => self.alu::<BASE>(f, d, |_, x| x.rotate_left(4)),
            Decfsz(f, d) => {
                let mut zero = false;
                let c = self.alu::<BASE>(f, d, |_, x| {
                    let r = x.wrapping_sub(1);
                    zero = r == 0;
                    r
                });
                c + self.skip::<BASE>(zero)
            }
            Incfsz(f, d) => {
                let mut zero = false;
                let c = self.alu::<BASE>(f, d, |_, x| {
                    let r = x.wrapping_add(1);
                    zero = r == 0;
                    r
                });
                c + self.skip::<BASE>(zero)
            }
            Bcf(f, b) => {
                let a = self.file::<BASE>(f);
                let v = self.rd::<BASE>(a) & !(1 << b);
                self.wr::<BASE>(a, v);
                self.pcl_cycles::<BASE>(a)
            }
            Bsf(f, b) => {
                let a = self.file::<BASE>(f);
                let v = self.rd::<BASE>(a) | (1 << b);
                self.wr::<BASE>(a, v);
                self.pcl_cycles::<BASE>(a)
            }
            Btfsc(f, b) => {
                let a = self.file::<BASE>(f);
                let clear = self.rd::<BASE>(a) & (1 << b) == 0;
                1 + self.skip::<BASE>(clear)
            }
            Btfss(f, b) => {
                let a = self.file::<BASE>(f);
                let set = self.rd::<BASE>(a) & (1 << b) != 0;
                1 + self.skip::<BASE>(set)
            }
            Call(k) => {
                if BASE {
                    self.push12(self.pc);
                    // CALL's 8-bit target clears PC<8>: subroutines start in
                    // the first 256 words.
                    self.pc = k & 0xFF;
                } else {
                    self.push(self.pc);
                    self.pc = self.page(k);
                }
                2
            }
            Goto(k) => {
                self.pc = if BASE {
                    k & baseline::PC_MASK
                } else {
                    self.page(k)
                };
                2
            }
            Return => {
                self.pc = self.ret::<BASE>();
                2
            }
            Retfie => {
                self.pc = self.ret::<BASE>();
                self.regs[INTCON as usize] |= 1 << GIE;
                2
            }
            Retlw(k) => {
                self.w = k;
                self.pc = self.ret::<BASE>();
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
            // Only the baseline decodes these. OPTION and TRIS live where the
            // 12F675 keeps OPTION_REG and TRISIO; see `baseline`.
            Option => {
                self.regs[OPTION_REG as usize] = self.w;
                1
            }
            Tris(f) => {
                if f == baseline::GPIO {
                    // GP3 is input-only.
                    self.regs[TRISIO as usize] = self.w | 0x08;
                }
                1
            }
        }
    }

    /// The register an instruction's file operand names: on the midrange its
    /// canonical address (or [`UNIMPLEMENTED`]), on the baseline the 5-bit
    /// operand itself.
    #[inline(always)]
    fn file<const BASE: bool>(&self, f: u8) -> u8 {
        if BASE {
            f & 0x1F
        } else {
            CANONICAL[self.direct(f) as usize]
        }
    }

    #[inline(always)]
    fn rd<const BASE: bool>(&mut self, a: u8) -> u8 {
        if BASE {
            self.read12(a)
        } else {
            self.read_reg(a)
        }
    }

    #[inline(always)]
    fn wr<const BASE: bool>(&mut self, a: u8, v: u8) {
        if BASE {
            self.write12(a, v)
        } else {
            self.write_reg(a, v)
        }
    }

    /// A byte-oriented file instruction: read f, compute, and store the result
    /// in W (`d` false) or back in f.
    #[inline(always)]
    fn alu<const BASE: bool>(
        &mut self,
        f: u8,
        d: bool,
        op: impl FnOnce(&mut Self, u8) -> u8,
    ) -> u32 {
        let a = self.file::<BASE>(f);
        let x = self.rd::<BASE>(a);
        let r = op(self, x);
        if d {
            if a == STATUS {
                // The flags the instruction set win over the written value.
                let flags = self.status() & 0x07;
                self.wr::<BASE>(a, r);
                let s = &mut self.regs[STATUS as usize];
                *s = (*s & !0x07) | flags;
            } else {
                self.wr::<BASE>(a, r);
            }
            self.pcl_cycles::<BASE>(a)
        } else {
            self.w = r;
            1
        }
    }

    #[inline(always)]
    fn zero(&mut self, r: u8) -> u8 {
        self.set_flag(Z, r == 0);
        r
    }

    /// A write to PCL is a jump and costs the second cycle.
    #[inline(always)]
    fn pcl_cycles<const BASE: bool>(&self, a: u8) -> u32 {
        if a == PCL { 2 } else { 1 }
    }

    #[inline(always)]
    fn skip<const BASE: bool>(&mut self, take: bool) -> u32 {
        if take {
            let mask = if BASE { baseline::PC_MASK } else { 0x1FFF };
            self.pc = (self.pc + 1) & mask;
            1
        } else {
            0
        }
    }

    #[inline(always)]
    /// The return address, from whichever stack the core has.
    fn ret<const BASE: bool>(&mut self) -> u16 {
        if BASE { self.pop12() } else { self.pop() }
    }

    /// GOTO/CALL take PCLATH<4:3> as the page bits above their 11-bit target.
    #[inline(always)]
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
    ///
    /// Within one instruction's cycles the peripherals do not affect each
    /// other, unless the watchdog fires and resets the part between them, and
    /// nothing moves a pin, so only the first cycle can see one change. So
    /// when the watchdog cannot fire, each is advanced over all `n` cycles at
    /// once, its enable bits read once rather than per cycle; otherwise they
    /// go cycle by cycle, in order.
    #[inline(always)]
    fn tick(&mut self, n: u32) -> u32 {
        if self.config.0 & 0x08 != 0 && self.wdt + n as u64 >= self.wdt_limit() {
            return self.tick_cycles(n);
        }
        self.cycles += n as u64;
        if self.config.0 & 0x08 != 0 {
            self.wdt += n as u64;
        }
        if !self.sleeping {
            self.advance_tmr0::<true>(n, Self::tick_tmr0);
            self.advance_tmr1(n);
        }
        if self.adc_busy > 0 {
            if self.adc_busy <= n {
                self.adc_busy = 0;
                self.finish_adc();
            } else {
                self.adc_busy -= n;
            }
        }
        self.watch_pins();
        n
    }

    /// [`Pic::tick`] one cycle at a time.
    #[cold]
    fn tick_cycles(&mut self, n: u32) -> u32 {
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

    /// TMR0 over `n` cycles while awake, setting T0IF when it wraps if
    /// `FLAG` (the 12C508 has no flag). `per_cycle` is the core's one-cycle
    /// rule, for what the bulk count does not cover.
    #[inline(always)]
    pub(super) fn advance_tmr0<const FLAG: bool>(&mut self, n: u32, per_cycle: fn(&mut Self)) {
        let option = self.regs[OPTION_REG as usize];
        let shift = (option & 0x07) + 1;
        let rate = 1u16 << shift;
        // Counting T0CKI edges, or a prescaler already past a rate just
        // lowered: the per-cycle rules handle those.
        if option & 0x20 != 0 || (option & 0x08 == 0 && self.prescaler >= rate) {
            for _ in 0..n {
                per_cycle(self);
            }
            return;
        }
        let held = (self.tmr0_hold as u32).min(n);
        self.tmr0_hold -= held as u8;
        let mut counts = n - held;
        if option & 0x08 == 0 {
            let total = self.prescaler as u32 + counts;
            counts = total >> shift;
            self.prescaler = (total & (rate as u32 - 1)) as u16;
        }
        if counts > 0 {
            let t = self.regs[TMR0 as usize] as u32 + counts;
            self.regs[TMR0 as usize] = t as u8;
            if FLAG && t > 0xFF {
                self.regs[INTCON as usize] |= 1 << T0IF;
            }
        }
    }

    /// TMR1 over `n` cycles while awake.
    #[inline(always)]
    fn advance_tmr1(&mut self, n: u32) {
        let t1con = self.regs[T1CON as usize];
        if t1con & 0x01 == 0 || t1con & 0x02 != 0 {
            return;
        }
        for _ in 0..n {
            self.tick_tmr1();
        }
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
            let level = self.level14(2);
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
        if t1con & 0x40 != 0 && self.level14(4) {
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

    /// The watchdog count at which it times out, with the postscaler.
    #[inline(always)]
    pub(super) fn wdt_limit(&self) -> u64 {
        let option = self.regs[OPTION_REG as usize];
        let post = if option & 0x08 != 0 {
            1u64 << (option & 0x07)
        } else {
            1
        };
        WDT_PERIOD * post
    }

    fn tick_wdt(&mut self) {
        if self.config.0 & 0x08 == 0 {
            return;
        }
        self.wdt += 1;
        if self.wdt < self.wdt_limit() {
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
            self.reset14();
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
    #[inline(always)]
    fn watch_pins(&mut self) {
        if self.pins_settled {
            return;
        }
        let levels = self.levels14();
        let level = levels & 0x04 != 0;
        if level != self.int_level {
            let rising = self.regs[OPTION_REG as usize] & 0x40 != 0;
            if level == rising {
                self.regs[INTCON as usize] |= 1 << INTF;
            }
            self.int_level = level;
        }
        let ioc = self.regs[IOC as usize];
        let changed = ((levels & !self.analog14()) ^ self.gpio_latched) & ioc != 0;
        if changed {
            self.regs[INTCON as usize] |= 1 << GPIF;
        }
        // While a change stands, GPIF is set again every cycle, so software
        // clearing it sees it come back.
        self.pins_settled = !changed;
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
        match self.part.core() {
            Core::Midrange => pin != 3 && self.regs[TRISIO as usize] & (1 << pin) == 0,
            Core::Baseline => self.is_output12(pin),
        }
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
            pic.rom[i] =
                assemble(line, &names, Part::Pic12F675).unwrap_or_else(|e| panic!("{line}: {e}"));
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
        pic.run(400, || false);
        assert_eq!(pic.pc, 4);
        assert!(pic.peek(INTCON) & (1 << T0IF) != 0);
        assert!(pic.peek(INTCON) & (1 << GIE) == 0);
    }

    #[test]
    fn breakpoints_stop_a_run() {
        let mut pic = load(&["nop", "nop", "goto 0"]);
        pic.breakpoints[2] = true;
        assert_eq!(pic.run(1000, || false), Stop::Breakpoint(2));
        // Resuming from it does not stop at once.
        assert_eq!(pic.run(1000, || false), Stop::Breakpoint(2));
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

    #[test]
    fn plain_writes_are_the_ones_with_no_special_rule() {
        let mut pic = Pic::new();
        pic.regs[FSR as usize] = 0x20;
        for a in 0..=255u8 {
            if !PLAIN_WRITE[a as usize] {
                continue;
            }
            let mut special = pic.clone();
            special.write_special(a, 0x5A);
            let mut plain = pic.clone();
            plain.regs[a as usize] = 0x5A;
            assert_eq!(state(&special), state(&plain), "register 0x{a:02X}");
        }
    }

    /// What `run` did before it batched cycles: one `step` at a time.
    fn run_by_steps(pic: &mut Pic, budget: u64) -> Stop {
        let words = pic.rom.len();
        let end = pic.cycles + budget;
        let mut steps = 0;
        while pic.cycles < end {
            if steps > 0 && pic.breakpoints[pic.pc as usize % words] {
                return Stop::Breakpoint(pic.pc);
            }
            steps += 1;
            pic.step();
        }
        if pic.breakpoints[pic.pc as usize % words] {
            return Stop::Breakpoint(pic.pc);
        }
        Stop::Budget
    }

    fn state(pic: &Pic) -> (u16, u8, u64, bool, Vec<u16>, Vec<u8>, u32) {
        let regs = (0..=255).map(|a| pic.peek(a)).collect();
        let wdt = pic.wdt_fraction().to_bits();
        (
            pic.pc,
            pic.w,
            pic.cycles,
            pic.sleeping,
            pic.stack_view(),
            regs,
            wdt,
        )
    }

    /// `run`, with its batched timers and sleep, lands exactly where stepping
    /// does, in chunks of odd sizes and with pins changing between them.
    fn assert_run_matches_stepping(src: &str) {
        let mut fast = Pic::new();
        crate::project::assemble_source(src, &mut fast).unwrap();
        let mut slow = fast.clone();
        for chunk in 0..400u64 {
            let budget = 1 + chunk * 7919 % 5000;
            assert_eq!(fast.run(budget, || false), run_by_steps(&mut slow, budget));
            assert_eq!(state(&fast), state(&slow), "after chunk {chunk}");
            if chunk % 37 == 0 {
                // Each pin in turn, high on the first round and low on the next.
                let k = chunk / 37;
                let (pin, high) = ((k % 6) as u8, k / 6 % 2 == 0);
                fast.drive(pin, high);
                slow.drive(pin, high);
            }
        }
    }

    #[test]
    fn run_matches_stepping_for_the_examples() {
        for file in [
            "blink.asm",
            "counter.asm",
            "blink-12c508.asm",
            "counter-12c508.asm",
        ] {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
            assert_run_matches_stepping(&std::fs::read_to_string(path.join(file)).unwrap());
        }
    }

    #[test]
    fn run_matches_stepping_as_pins_change_under_interrupt_flags() {
        // GP2 flips between an output driving high and an input floating low,
        // for the INT edge; GP3's change interrupt stands while GPIO goes
        // unread, so GPIF comes back after each clear.
        assert_run_matches_stepping(
            " __config 0x3F94
 movlw 7
 movwf CMCON
 bsf STATUS,RP0
 clrf ANSEL
 movlw 0x08
 movwf IOC
 bcf STATUS,RP0
 movlw 0x04
 movwf GPIO
again bsf STATUS,RP0
 movlw 0x3B
 movwf TRISIO
 bcf STATUS,RP0
 incf 0x20,f
 bsf STATUS,RP0
 movlw 0x3F
 movwf TRISIO
 bcf STATUS,RP0
 btfsc INTCON,1
 incf 0x21,f
 btfsc INTCON,0
 incf 0x22,f
 bcf INTCON,1
 bcf INTCON,0
 decfsz 0x23,f
 goto again
 movf GPIO,w
 goto again",
        );
    }

    #[test]
    fn run_matches_stepping_asleep_until_the_adc_or_watchdog() {
        // Sleep through an A/D conversion, woken by its interrupt flag.
        assert_run_matches_stepping(
            " __config 0x3F94
 bsf STATUS,RP0
 bsf PIE1,6
 bcf STATUS,RP0
 bsf INTCON,6
again movlw 0x03
 movwf ADCON0
 sleep
 bcf PIR1,6
 incf 0x20,f
 goto again",
        );
        // Sleep until the watchdog wakes it, with the postscaler on it.
        assert_run_matches_stepping(
            " __config 0x3F9C
 bsf STATUS,RP0
 movlw 0x09
 movwf OPTION_REG
 bcf STATUS,RP0
again sleep
 incf 0x20,f
 goto again",
        );
        // On the 12C508 the watchdog wakes it with a reset.
        assert_run_matches_stepping(
            " processor 12C508
 __config 0xFEE
 sleep",
        );
    }
}
