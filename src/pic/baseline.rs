//! The PIC12C508: the 12-bit baseline core's register file and peripherals
//! (datasheet DS40139). The instructions themselves run in [`Pic::exec`],
//! shared with the 12F675.
//!
//! The baseline is the midrange's smaller ancestor: 512 words of program, 25
//! bytes of RAM in one bank, a 2-level stack, and no interrupts. OPTION and the
//! GPIO direction are written by the OPTION and TRIS instructions and have no
//! file address. The part starts at the last word, where the factory left a
//! MOVLW of the oscillator calibration, and wraps to 0. Every wake from SLEEP
//! is a reset: STATUS says which kind (GPWUF, /TO, /PD), and RAM survives it.
//!
//! Modelled: all 33 instructions; INDF/FSR; PCL computed jumps (which, like
//! CALL, reach only the first 256 words); the 2-level stack; TMR0 with its
//! prescaler and T0CKI on GP2; the watchdog with postscaler; SLEEP with wake
//! on a change of GP0, GP1 or GP3; GPIO with TRIS and the weak pull-ups on
//! GP0, GP1 and GP3; OSCCAL. Not modelled: GP3 as /MCLR, GP4/GP5 as an
//! external oscillator's pins, and the device reset timer's delay on wake.

use super::{FSR, INDF, OPTION_REG, PCL, Pic, STATUS, TMR0, TRISIO, WDT_PERIOD};

pub const ROM_WORDS: usize = 512;
/// The erased state of a program word: `XORLW 0xFF`.
pub const ERASED: u16 = 0xFFF;
pub(super) const PC_MASK: u16 = ROM_WORDS as u16 - 1;
/// Where the part starts: the last word, holding the calibration.
pub const RESET_VECTOR: u16 = PC_MASK;
/// The factory's last word: `MOVLW 0x70`, the oscillator's trim value for
/// OSCCAL.
pub(super) const CALIBRATION: u16 = 0xC70;
/// Internal oscillator, watchdog off, no code protection, GP3 an input
/// rather than /MCLR: MPASM's `_IntRC_OSC & _WDT_OFF & _CP_OFF & _MCLRE_OFF`.
pub(super) const DEFAULT_CONFIG: super::Config = super::Config(0xFEA);

pub const OSCCAL: u8 = 0x05;
pub const GPIO: u8 = 0x06;

pub const SFRS: &[(u8, &str)] = &[
    (INDF, "INDF"),
    (TMR0, "TMR0"),
    (PCL, "PCL"),
    (STATUS, "STATUS"),
    (FSR, "FSR"),
    (OSCCAL, "OSCCAL"),
    (GPIO, "GPIO"),
];

pub const GPRS: std::ops::RangeInclusive<u8> = 0x07..=0x1F;

// STATUS bits beyond the midrange's C, DC, Z, PD and TO.
pub const PA0: u8 = 5;
pub const GPWUF: u8 = 7;

// OPTION bits that differ from the midrange's OPTION_REG.
/// Clear to wake from SLEEP on a change of GP0, GP1 or GP3.
const NOT_GPWU: u8 = 0x80;
/// Clear to turn on the weak pull-ups.
const NOT_GPPU: u8 = 0x40;
const T0CS: u8 = 0x20;

/// The pins with a weak pull-up and wake-up on change: GP0, GP1, GP3.
const WAKE_PINS: u8 = 0x0B;

/// OPTION and TRIS have no file address on the baseline, and a 5-bit operand
/// cannot reach past 0x1F, so they are kept where the 12F675 keeps OPTION_REG
/// and TRISIO. [`Pic::option`] and [`Pic::tris`] read them for display.
const OPTION: usize = OPTION_REG as usize;
const TRIS: usize = TRISIO as usize;

/// What reset the part, which STATUS records for the program to test.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Cause {
    PowerOn,
    /// The watchdog ran out while the part was awake.
    Watchdog,
    /// The watchdog woke the part from SLEEP.
    WatchdogWake,
    /// A pin change woke the part from SLEEP.
    PinWake,
}

impl Pic {
    pub(super) fn reset12(&mut self, cause: Cause) {
        let (gpwuf, to, pd) = match cause {
            Cause::PowerOn => (false, true, true),
            Cause::Watchdog => (false, false, true),
            Cause::WatchdogWake => (false, false, false),
            Cause::PinWake => (true, true, false),
        };
        if cause == Cause::PowerOn {
            self.regs = [0; 256];
            self.regs[OSCCAL as usize] = 0x70;
            self.w = 0;
            self.cycles = 0;
        }
        // Any other reset keeps RAM, W, TMR0, FSR, OSCCAL, the GPIO latch and
        // the arithmetic flags.
        let flags = self.regs[STATUS as usize] & 0x07;
        self.regs[STATUS as usize] =
            (gpwuf as u8) << GPWUF | (to as u8) << super::TO | (pd as u8) << super::PD | flags;
        self.regs[OPTION] = 0xFF;
        self.regs[TRIS] = 0x3F;
        self.pc = RESET_VECTOR;
        self.stack = [0; super::STACK_DEPTH];
        self.sp = 0;
        self.depth = 0;
        self.sleeping = false;
        self.prescaler = 0;
        self.wdt = 0;
        self.tmr0_hold = 0;
        self.t0ck_level = self.level12(2);
        self.gpio_latched = self.gpio12();
    }

    // ---- register file -------------------------------------------------

    pub(super) fn peek12(&self, a: u8) -> u8 {
        match a {
            INDF => {
                let f = self.regs[FSR as usize] & 0x1F;
                if f == INDF { 0 } else { self.peek12(f) }
            }
            PCL => self.pc as u8,
            // FSR<7:5> are unimplemented and read as 1.
            FSR => self.regs[FSR as usize] | 0xE0,
            GPIO => self.gpio12(),
            _ if a <= 0x1F => self.regs[a as usize],
            _ => 0,
        }
    }

    /// A read with its side effect: reading GPIO sets the reference that
    /// wake-on-change compares against.
    pub(super) fn read12(&mut self, a: u8) -> u8 {
        let v = self.peek12(a);
        if a == GPIO {
            self.gpio_latched = v;
        }
        v
    }

    pub(super) fn write12(&mut self, a: u8, v: u8) {
        let r = &mut self.regs;
        match a {
            INDF => {
                let f = r[FSR as usize] & 0x1F;
                if f != INDF {
                    self.write12(f, v);
                }
            }
            TMR0 => {
                r[TMR0 as usize] = v;
                // A write clears an assigned prescaler and holds the count.
                if r[OPTION] & 0x08 == 0 {
                    self.prescaler = 0;
                }
                self.tmr0_hold = 2;
            }
            // A computed jump: PC<8> is cleared, as for CALL.
            PCL => self.pc = v as u16,
            STATUS => {
                // /TO and /PD are read-only and bit 6 is unimplemented.
                r[STATUS as usize] = (v & 0xA7) | (r[STATUS as usize] & 0x18);
            }
            FSR => r[FSR as usize] = v & 0x1F,
            OSCCAL => r[OSCCAL as usize] = v & 0xF0,
            GPIO => r[GPIO as usize] = v & 0x37,
            0x07..=0x1F => r[a as usize] = v,
            _ => {}
        }
    }

    // ---- pins ----------------------------------------------------------

    /// TRIS as the pins see it: T0CKI makes GP2 an input whatever TRIS says,
    /// and GP3 is always one.
    fn inputs12(&self) -> u8 {
        let mut tris = self.regs[TRIS] | 0x08;
        if self.regs[OPTION] & T0CS != 0 {
            tris |= 0x04;
        }
        tris
    }

    pub(super) fn is_output12(&self, pin: u8) -> bool {
        self.inputs12() & (1 << pin) == 0
    }

    pub(super) fn level12(&self, pin: u8) -> bool {
        let bit = 1 << pin;
        if self.inputs12() & bit == 0 {
            return self.regs[GPIO as usize] & bit != 0;
        }
        if self.pins_driven & bit != 0 {
            return self.pins_in & bit != 0;
        }
        self.regs[OPTION] & NOT_GPPU == 0 && WAKE_PINS & bit != 0
    }

    fn gpio12(&self) -> u8 {
        (0..6)
            .filter(|&pin| self.level12(pin))
            .fold(0, |v, pin| v | 1 << pin)
    }

    // ---- stack ---------------------------------------------------------

    /// Two levels and no pointer: a push moves the top down and the bottom
    /// falls off; a pop moves the bottom up and leaves a copy of it there.
    pub(super) fn push12(&mut self, addr: u16) {
        self.stack[1] = self.stack[0];
        self.stack[0] = addr & PC_MASK;
        self.depth = (self.depth + 1).min(2);
    }

    pub(super) fn pop12(&mut self) -> u16 {
        let top = self.stack[0];
        self.stack[0] = self.stack[1];
        self.depth = self.depth.saturating_sub(1);
        top
    }

    // ---- execution -----------------------------------------------------

    pub(super) fn step12(&mut self) -> u32 {
        if self.sleeping {
            return self.tick12(1);
        }
        let word = self.rom[self.pc as usize % ROM_WORDS];
        self.pc = (self.pc + 1) & PC_MASK;
        let cycles = self.exec::<true>(crate::isa::decode12(word));
        self.tick12(cycles)
    }

    fn tick12(&mut self, n: u32) -> u32 {
        for _ in 0..n {
            self.cycles += 1;
            self.tick_tmr0_12();
            self.tick_wdt12();
            if self.sleeping
                && self.regs[OPTION] & NOT_GPWU == 0
                && (self.gpio12() ^ self.gpio_latched) & WAKE_PINS != 0
            {
                self.reset12(Cause::PinWake);
            }
        }
        n
    }

    /// TMR0 as on the 12F675, but nothing is flagged when it wraps.
    fn tick_tmr0_12(&mut self) {
        if self.sleeping {
            return;
        }
        let option = self.regs[OPTION];
        let edge = if option & T0CS == 0 {
            true
        } else {
            let level = self.level12(2);
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
    }

    /// The watchdog as on the 12F675, but a timeout is always a reset, in
    /// SLEEP or out of it.
    fn tick_wdt12(&mut self) {
        if !self.wdt_enabled() {
            return;
        }
        self.wdt += 1;
        let option = self.regs[OPTION];
        let post = if option & 0x08 != 0 {
            1u64 << (option & 0x07)
        } else {
            1
        };
        if self.wdt < WDT_PERIOD * post {
            return;
        }
        let cause = if self.sleeping {
            Cause::WatchdogWake
        } else {
            Cause::Watchdog
        };
        self.reset12(cause);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isa::{Names, assemble};
    use crate::pic::{C, DC, PD, Part, Stop, TO, Z};

    fn load(src: &[&str]) -> Pic {
        let mut pic = Pic::with_part(Part::Pic12C508);
        let names = Names::default();
        for (i, line) in src.iter().enumerate() {
            pic.rom[i] =
                assemble(line, &names, Part::Pic12C508).unwrap_or_else(|e| panic!("{line}: {e}"));
        }
        pic
    }

    /// Run until the PC first reaches `addr`.
    fn run_to(pic: &mut Pic, addr: u16) {
        for _ in 0..10_000 {
            if pic.pc == addr && !pic.sleeping {
                return;
            }
            pic.step();
        }
        panic!("never reached 0x{addr:03X}; PC 0x{:03X}", pic.pc);
    }

    #[test]
    fn starts_at_the_calibration_word_and_wraps_to_zero() {
        let mut pic = load(&["movwf OSCCAL"]);
        assert_eq!(pic.pc, 0x1FF);
        assert_eq!(pic.rom[0x1FF], CALIBRATION);
        pic.step();
        assert_eq!((pic.pc, pic.w), (0, 0x70));
        pic.step();
        assert_eq!(pic.peek(OSCCAL), 0x70);
        assert_eq!(pic.status() & 0x18, 0x18, "/TO and /PD set at power-on");
    }

    #[test]
    fn arithmetic_matches_the_midrange() {
        let mut pic = load(&["movlw 0x88", "movwf 0x10", "movlw 0x78", "addwf 0x10,f"]);
        run_to(&mut pic, 4);
        assert_eq!(pic.peek(0x10), 0);
        assert!(pic.flag(C) && pic.flag(DC) && pic.flag(Z));
    }

    #[test]
    fn option_and_tris_set_up_gpio() {
        let mut pic = load(&[
            "movlw 0xDF",
            "option", // T0CS off: GP2 is free for I/O
            "movlw 0x3A",
            "tris GPIO", // GP0 and GP2 out
            "movlw 0x05",
            "movwf GPIO",
        ]);
        run_to(&mut pic, 6);
        assert_eq!(pic.option(), 0xDF);
        assert_eq!(pic.tris(), 0x3A);
        assert!(pic.is_output(0) && pic.is_output(2) && !pic.is_output(1));
        assert!(pic.pin_level(0) && pic.pin_level(2));
        assert_eq!(pic.peek(GPIO) & 0x05, 0x05);
        // At power-on T0CS is set, so GP2 is T0CKI and an input.
        let pic = Pic::with_part(Part::Pic12C508);
        assert!(!pic.is_output(2));
    }

    #[test]
    fn the_stack_has_two_levels_and_the_bottom_falls_off() {
        // Three nested calls: the return from the third comes back to the
        // second, and the next return goes there again.
        let mut pic = load(&[
            "call 4",    // 0
            "goto 0x0F", // 1
            "nop",       // 2
            "nop",       // 3
            "call 6",    // 4
            "retlw 4",   // 5
            "call 8",    // 6
            "retlw 6",   // 7
            "retlw 8",   // 8
        ]);
        run_to(&mut pic, 8);
        assert_eq!(pic.stack_view(), vec![7, 5]);
        pic.step(); // retlw 8 -> 7
        assert_eq!((pic.pc, pic.w), (7, 8));
        pic.step(); // retlw 6 -> 5
        assert_eq!((pic.pc, pic.w), (5, 6));
        pic.step(); // retlw 4 -> 5 again: the call from 0 fell off
        assert_eq!((pic.pc, pic.w), (5, 4));
    }

    #[test]
    fn call_and_computed_jumps_reach_only_the_first_256_words() {
        let mut names = Names::default();
        names.labels.insert(0x150, "far".into());
        assert!(assemble("call far", &names, Part::Pic12C508).is_err());
        assert!(assemble("goto far", &names, Part::Pic12C508).is_ok());
        let mut pic = load(&["goto 0x150"]);
        pic.rom[0x150] = assemble("movlw 3", &names, Part::Pic12C508).unwrap();
        pic.rom[0x151] = assemble("movwf PCL", &names, Part::Pic12C508).unwrap();
        run_to(&mut pic, 0x151);
        pic.step();
        assert_eq!(pic.pc, 3, "PC<8> cleared");
    }

    #[test]
    fn table_lookup_with_retlw() {
        let mut pic = load(&[
            "movlw 2",
            "call 4",
            "movwf 0x07",
            "goto 3",
            "addwf PCL,f",
            "retlw 10",
            "retlw 20",
            "retlw 30",
        ]);
        run_to(&mut pic, 3);
        assert_eq!(pic.peek(0x07), 30);
    }

    #[test]
    fn indirect_addressing_and_fsr_high_bits() {
        let mut pic = load(&[
            "movlw 0x09",
            "movwf FSR",
            "movlw 0x42",
            "movwf INDF",
            "incf INDF,f",
        ]);
        run_to(&mut pic, 5);
        assert_eq!(pic.peek(0x09), 0x43);
        assert_eq!(pic.peek(FSR), 0xE9);
    }

    #[test]
    fn tmr0_counts_with_no_flag() {
        // Prescaler to the watchdog: TMR0 counts every cycle and wraps.
        let mut pic = load(&["movlw 0xCF", "option", "goto 2"]);
        pic.run(300, &|| false);
        assert!(pic.peek(TMR0) < 60);
        assert_eq!(pic.peek(0x0B), 0, "no INTCON to set a flag in");
    }

    #[test]
    fn wake_on_pin_change_is_a_reset_that_keeps_ram() {
        let mut pic = load(&[
            "movlw 0x0F", // 0: wake-on-change and pull-ups on
            "option",
            "incf 0x10,f", // 2: count every start
            "movf GPIO,w", // 3: the reference for the change
            "sleep",       // 4
            "goto 0",      // 5: never runs
        ]);
        pic.run(50, &|| false);
        assert!(pic.sleeping);
        assert_eq!(pic.peek(0x10), 1);
        assert!(!pic.flag(PD) && pic.flag(TO));
        // GP3 is pulled up; pressing a button pulls it low.
        pic.drive(3, false);
        pic.step();
        assert!(!pic.sleeping);
        assert_eq!(pic.pc, RESET_VECTOR);
        assert_eq!(pic.status() >> GPWUF & 1, 1);
        assert_eq!(pic.option(), 0xFF, "OPTION back to its reset value");
        run_to(&mut pic, 4);
        assert_eq!(pic.peek(0x10), 2, "RAM kept across the wake");
    }

    #[test]
    fn watchdog_resets_and_wakes() {
        // The prescaler goes to TMR0, so the watchdog has no postscaler; a
        // breakpoint on the reset vector catches the reset.
        for (last, asleep) in [("goto 2", false), ("sleep", true)] {
            let mut pic = load(&["movlw 0xC0", "option", last]);
            pic.config = crate::pic::Config(0xFEE); // WDT on
            assert!(pic.wdt_enabled());
            pic.breakpoints[RESET_VECTOR as usize] = true;
            let stop = pic.run(WDT_PERIOD + 10, &|| false);
            assert_eq!(stop, Stop::Breakpoint(RESET_VECTOR));
            assert!(!pic.flag(TO), "/TO clear after a timeout");
            assert_eq!(pic.flag(PD), !asleep, "/PD clear only if it slept");
            assert!(!pic.sleeping);
        }
    }

    #[test]
    fn breakpoints_stop_a_run() {
        let mut pic = load(&["nop", "nop", "goto 0"]);
        pic.breakpoints[2] = true;
        assert_eq!(pic.run(1000, &|| false), Stop::Breakpoint(2));
    }
}
