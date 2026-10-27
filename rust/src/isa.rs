//! The 14-bit midrange instruction set: decode, encode, and the one-line
//! assembler and disassembler the program listing edits through.
//!
//! The old app found an instruction for typed text by string-matching it
//! against the mnemonic of every one of the 16,384 opcodes and keeping the
//! longest prefix match. Here the text is parsed: a mnemonic, then its
//! operands, each of which may be a number, a register or label name, or a
//! bit name.

use std::collections::BTreeMap;

use crate::pic;

/// One decoded instruction. `f` is the 7-bit file address, `d` true means the
/// result goes back to the file (`,f`), `b` is a bit number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Addwf(u8, bool),
    Andwf(u8, bool),
    Clrf(u8),
    Clrw,
    Comf(u8, bool),
    Decf(u8, bool),
    Decfsz(u8, bool),
    Incf(u8, bool),
    Incfsz(u8, bool),
    Iorwf(u8, bool),
    Movf(u8, bool),
    Movwf(u8),
    Nop,
    Rlf(u8, bool),
    Rrf(u8, bool),
    Subwf(u8, bool),
    Swapf(u8, bool),
    Xorwf(u8, bool),
    Bcf(u8, u8),
    Bsf(u8, u8),
    Btfsc(u8, u8),
    Btfss(u8, u8),
    Addlw(u8),
    Andlw(u8),
    Call(u16),
    Clrwdt,
    Goto(u16),
    Iorlw(u8),
    Movlw(u8),
    Retfie,
    Retlw(u8),
    Return,
    Sleep,
    Sublw(u8),
    Xorlw(u8),
}

/// What kind of operand an instruction takes — what the listing shows in its
/// operand column, and what the assembler expects after the mnemonic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    None,
    File,
    FileDest,
    FileBit,
    Literal,
    Address,
}

const MNEMONICS: &[(&str, Shape)] = &[
    ("ADDWF", Shape::FileDest),
    ("ANDWF", Shape::FileDest),
    ("CLRF", Shape::File),
    ("CLRW", Shape::None),
    ("COMF", Shape::FileDest),
    ("DECF", Shape::FileDest),
    ("DECFSZ", Shape::FileDest),
    ("INCF", Shape::FileDest),
    ("INCFSZ", Shape::FileDest),
    ("IORWF", Shape::FileDest),
    ("MOVF", Shape::FileDest),
    ("MOVWF", Shape::File),
    ("NOP", Shape::None),
    ("RLF", Shape::FileDest),
    ("RRF", Shape::FileDest),
    ("SUBWF", Shape::FileDest),
    ("SWAPF", Shape::FileDest),
    ("XORWF", Shape::FileDest),
    ("BCF", Shape::FileBit),
    ("BSF", Shape::FileBit),
    ("BTFSC", Shape::FileBit),
    ("BTFSS", Shape::FileBit),
    ("ADDLW", Shape::Literal),
    ("ANDLW", Shape::Literal),
    ("CALL", Shape::Address),
    ("CLRWDT", Shape::None),
    ("GOTO", Shape::Address),
    ("IORLW", Shape::Literal),
    ("MOVLW", Shape::Literal),
    ("RETFIE", Shape::None),
    ("RETLW", Shape::Literal),
    ("RETURN", Shape::None),
    ("SLEEP", Shape::None),
    ("SUBLW", Shape::Literal),
    ("XORLW", Shape::Literal),
];

pub fn decode(w: u16) -> Op {
    use Op::*;
    let w = w & 0x3FFF;
    let f = (w & 0x7F) as u8;
    let d = w & 0x80 != 0;
    let b = ((w >> 7) & 0x07) as u8;
    let k = (w & 0xFF) as u8;
    match w >> 12 {
        0b00 => match (w >> 8) & 0x0F {
            0x0 => {
                if d {
                    Movwf(f)
                } else {
                    match w {
                        0x0008 => Return,
                        0x0009 => Retfie,
                        0x0063 => Sleep,
                        0x0064 => Clrwdt,
                        _ => Nop,
                    }
                }
            }
            0x1 => {
                if d {
                    Clrf(f)
                } else {
                    Clrw
                }
            }
            0x2 => Subwf(f, d),
            0x3 => Decf(f, d),
            0x4 => Iorwf(f, d),
            0x5 => Andwf(f, d),
            0x6 => Xorwf(f, d),
            0x7 => Addwf(f, d),
            0x8 => Movf(f, d),
            0x9 => Comf(f, d),
            0xA => Incf(f, d),
            0xB => Decfsz(f, d),
            0xC => Rrf(f, d),
            0xD => Rlf(f, d),
            0xE => Swapf(f, d),
            _ => Incfsz(f, d),
        },
        0b01 => match (w >> 10) & 0x03 {
            0 => Bcf(f, b),
            1 => Bsf(f, b),
            2 => Btfsc(f, b),
            _ => Btfss(f, b),
        },
        0b10 => {
            let a = w & 0x7FF;
            if w & 0x0800 == 0 { Call(a) } else { Goto(a) }
        }
        _ => match (w >> 8) & 0x0F {
            0x0..=0x3 => Movlw(k),
            0x4..=0x7 => Retlw(k),
            0x8 => Iorlw(k),
            0x9 => Andlw(k),
            0xA => Xorlw(k),
            0xB => Nop, // unused encoding
            0xC | 0xD => Sublw(k),
            _ => Addlw(k),
        },
    }
}

pub fn encode(op: Op) -> u16 {
    use Op::*;
    let fd = |base: u16, f: u8, d: bool| base | ((d as u16) << 7) | (f as u16 & 0x7F);
    let fb = |base: u16, f: u8, b: u8| base | ((b as u16 & 7) << 7) | (f as u16 & 0x7F);
    match op {
        Nop => 0x0000,
        Return => 0x0008,
        Retfie => 0x0009,
        Sleep => 0x0063,
        Clrwdt => 0x0064,
        Movwf(f) => 0x0080 | (f as u16 & 0x7F),
        Clrw => 0x0100,
        Clrf(f) => 0x0180 | (f as u16 & 0x7F),
        Subwf(f, d) => fd(0x0200, f, d),
        Decf(f, d) => fd(0x0300, f, d),
        Iorwf(f, d) => fd(0x0400, f, d),
        Andwf(f, d) => fd(0x0500, f, d),
        Xorwf(f, d) => fd(0x0600, f, d),
        Addwf(f, d) => fd(0x0700, f, d),
        Movf(f, d) => fd(0x0800, f, d),
        Comf(f, d) => fd(0x0900, f, d),
        Incf(f, d) => fd(0x0A00, f, d),
        Decfsz(f, d) => fd(0x0B00, f, d),
        Rrf(f, d) => fd(0x0C00, f, d),
        Rlf(f, d) => fd(0x0D00, f, d),
        Swapf(f, d) => fd(0x0E00, f, d),
        Incfsz(f, d) => fd(0x0F00, f, d),
        Bcf(f, b) => fb(0x1000, f, b),
        Bsf(f, b) => fb(0x1400, f, b),
        Btfsc(f, b) => fb(0x1800, f, b),
        Btfss(f, b) => fb(0x1C00, f, b),
        Call(k) => 0x2000 | (k & 0x7FF),
        Goto(k) => 0x2800 | (k & 0x7FF),
        Movlw(k) => 0x3000 | k as u16,
        Retlw(k) => 0x3400 | k as u16,
        Iorlw(k) => 0x3800 | k as u16,
        Andlw(k) => 0x3900 | k as u16,
        Xorlw(k) => 0x3A00 | k as u16,
        Sublw(k) => 0x3C00 | k as u16,
        Addlw(k) => 0x3E00 | k as u16,
    }
}

impl Op {
    pub fn mnemonic(self) -> &'static str {
        use Op::*;
        match self {
            Addwf(..) => "ADDWF",
            Andwf(..) => "ANDWF",
            Clrf(_) => "CLRF",
            Clrw => "CLRW",
            Comf(..) => "COMF",
            Decf(..) => "DECF",
            Decfsz(..) => "DECFSZ",
            Incf(..) => "INCF",
            Incfsz(..) => "INCFSZ",
            Iorwf(..) => "IORWF",
            Movf(..) => "MOVF",
            Movwf(_) => "MOVWF",
            Nop => "NOP",
            Rlf(..) => "RLF",
            Rrf(..) => "RRF",
            Subwf(..) => "SUBWF",
            Swapf(..) => "SWAPF",
            Xorwf(..) => "XORWF",
            Bcf(..) => "BCF",
            Bsf(..) => "BSF",
            Btfsc(..) => "BTFSC",
            Btfss(..) => "BTFSS",
            Addlw(_) => "ADDLW",
            Andlw(_) => "ANDLW",
            Call(_) => "CALL",
            Clrwdt => "CLRWDT",
            Goto(_) => "GOTO",
            Iorlw(_) => "IORLW",
            Movlw(_) => "MOVLW",
            Retfie => "RETFIE",
            Retlw(_) => "RETLW",
            Return => "RETURN",
            Sleep => "SLEEP",
            Sublw(_) => "SUBLW",
            Xorlw(_) => "XORLW",
        }
    }
}

/// The names the listing shows and the assembler accepts: labels on program
/// addresses and names on registers, both the user's own. The SFR and bit
/// names of the part are always known and need not be added.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Names {
    pub labels: BTreeMap<u16, String>,
    pub registers: BTreeMap<u8, String>,
}

impl Names {
    /// The name shown for a 7-bit file operand in the given bank. A user name
    /// wins; otherwise that bank's SFR name, or the other bank's where this
    /// one has nothing there.
    pub fn file(&self, f: u8, bank1: bool) -> Option<String> {
        let f = f & 0x7F;
        if let Some(n) = self.registers.get(&f) {
            return Some(n.clone());
        }
        let (this, other) = if bank1 { (f | 0x80, f) } else { (f, f | 0x80) };
        pic::sfr_name(this)
            .or_else(|| pic::sfr_name(other))
            .map(str::to_string)
    }

    pub fn label(&self, addr: u16) -> Option<&str> {
        self.labels.get(&addr).map(String::as_str)
    }

    fn file_by_name(&self, name: &str) -> Option<u8> {
        let up = name.to_ascii_uppercase();
        if let Some((a, _)) = self
            .registers
            .iter()
            .find(|(_, n)| n.eq_ignore_ascii_case(name))
        {
            return Some(*a);
        }
        pic::SFRS
            .iter()
            .find(|(_, n)| *n == up || (up == "OPTION" && *n == "OPTION_REG"))
            .map(|(a, _)| a & 0x7F)
    }

    fn label_by_name(&self, name: &str) -> Option<u16> {
        self.labels
            .iter()
            .find(|(_, n)| n.eq_ignore_ascii_case(name))
            .map(|(a, _)| *a)
    }
}

/// Bit names, by the register they belong to. A bare bit name resolves
/// regardless of the register it is used with, as MPASM's include file does.
const BITS: &[(&str, u8)] = &[
    ("C", 0),
    ("DC", 1),
    ("Z", 2),
    ("NOT_PD", 3),
    ("PD", 3),
    ("NOT_TO", 4),
    ("TO", 4),
    ("RP0", 5),
    ("RP1", 6),
    ("IRP", 7),
    ("GPIF", 0),
    ("INTF", 1),
    ("T0IF", 2),
    ("GPIE", 3),
    ("INTE", 4),
    ("T0IE", 5),
    ("PEIE", 6),
    ("GIE", 7),
    ("TMR1IF", 0),
    ("CMIF", 3),
    ("ADIF", 6),
    ("EEIF", 7),
    ("TMR1IE", 0),
    ("CMIE", 3),
    ("ADIE", 6),
    ("EEIE", 7),
    ("TMR1ON", 0),
    ("TMR1CS", 1),
    ("NOT_T1SYNC", 2),
    ("T1OSCEN", 3),
    ("TMR1GE", 6),
    ("ADON", 0),
    ("GO", 1),
    ("GO_DONE", 1),
    ("ADFM", 7),
    ("RD", 0),
    ("WR", 1),
    ("WREN", 2),
    ("WRERR", 3),
    ("PS0", 0),
    ("PS1", 1),
    ("PS2", 2),
    ("PSA", 3),
    ("T0SE", 4),
    ("T0CS", 5),
    ("INTEDG", 6),
    ("NOT_GPPU", 7),
    ("GP0", 0),
    ("GP1", 1),
    ("GP2", 2),
    ("GP3", 3),
    ("GP4", 4),
    ("GP5", 5),
];

/// The listing's two text columns for one program word: the mnemonic and its
/// operands, with names substituted where there are any.
///
/// `bank1` is the register bank the listing assumes is selected here — see
/// [`banks`] — so `MOVWF 0x05` reads as TRISIO after a `BSF STATUS,RP0`.
pub fn disassemble(word: u16, names: &Names, bank1: bool) -> (&'static str, String) {
    use Op::*;
    let op = decode(word);
    let file = |f: u8| {
        names
            .file(f, bank1)
            .unwrap_or_else(|| format!("0x{:02X}", f & 0x7F))
    };
    let dest = |d: bool| if d { "f" } else { "w" };
    let addr = |k: u16| {
        names
            .label(k)
            .map(str::to_string)
            .unwrap_or_else(|| format!("0x{k:03X}"))
    };
    let operands = match op {
        Addwf(f, d)
        | Andwf(f, d)
        | Comf(f, d)
        | Decf(f, d)
        | Decfsz(f, d)
        | Incf(f, d)
        | Incfsz(f, d)
        | Iorwf(f, d)
        | Movf(f, d)
        | Rlf(f, d)
        | Rrf(f, d)
        | Subwf(f, d)
        | Swapf(f, d)
        | Xorwf(f, d) => format!("{},{}", file(f), dest(d)),
        Clrf(f) | Movwf(f) => file(f),
        Bcf(f, b) | Bsf(f, b) | Btfsc(f, b) | Btfss(f, b) => {
            format!("{},{}", file(f), bit_name(f, b))
        }
        Addlw(k) | Andlw(k) | Iorlw(k) | Movlw(k) | Retlw(k) | Sublw(k) | Xorlw(k) => {
            format!("0x{k:02X}")
        }
        Call(k) | Goto(k) => addr(k),
        Clrw | Nop | Clrwdt | Retfie | Return | Sleep => String::new(),
    };
    (op.mnemonic(), operands)
}

/// The register bank the listing assumes at each address, by reading the
/// program top to bottom and following `BSF/BCF STATUS,RP0`. It is a guess —
/// a jump can arrive in either bank — but it is the guess a person reading the
/// listing makes, and it names `TRISIO` and `OPTION_REG` where they are meant.
pub fn banks(rom: &[u16]) -> Vec<bool> {
    let mut bank1 = false;
    rom.iter()
        .map(|w| {
            let here = bank1;
            match decode(*w) {
                Op::Bsf(0x03, 5) => bank1 = true,
                Op::Bcf(0x03, 5) => bank1 = false,
                _ => {}
            }
            here
        })
        .collect()
}

/// A bit shown by name where the register has well-known ones.
fn bit_name(f: u8, b: u8) -> String {
    let names: &[&str] = match f & 0x7F {
        0x03 => &["C", "DC", "Z", "PD", "TO", "RP0", "RP1", "IRP"],
        0x0B => &[
            "GPIF", "INTF", "T0IF", "GPIE", "INTE", "T0IE", "PEIE", "GIE",
        ],
        _ => &[],
    };
    names
        .get(b as usize)
        .map(|s| s.to_string())
        .unwrap_or_else(|| b.to_string())
}

/// Assemble one line — `movlw 0x20`, `btfss STATUS,Z`, `goto loop` — into a
/// program word. A `;` starts a comment. Numbers are decimal unless written
/// `0x1F`, `1Fh`, `h'1F'`, `b'0101'`, `0b0101`, `d'10'` or `.10`.
pub fn assemble(line: &str, names: &Names) -> Result<u16, String> {
    use Op::*;
    let line = line.split(';').next().unwrap_or("").trim();
    if line.is_empty() {
        return Err("empty line".into());
    }
    let (mnem, rest) = match line.find(char::is_whitespace) {
        Some(i) => (&line[..i], line[i..].trim()),
        None => (line, ""),
    };
    let mnem = mnem.to_ascii_uppercase();
    let Some((_, shape)) = MNEMONICS.iter().find(|(m, _)| *m == mnem) else {
        return Err(format!("unknown instruction '{mnem}'"));
    };
    let args: Vec<&str> = if rest.is_empty() {
        Vec::new()
    } else {
        rest.split(',').map(str::trim).collect()
    };
    let want = match shape {
        Shape::None => 0..=0,
        Shape::File | Shape::Literal | Shape::Address => 1..=1,
        Shape::FileDest => 1..=2,
        Shape::FileBit => 2..=2,
    };
    if !want.contains(&args.len()) {
        return Err(format!("{mnem} takes {} operand(s)", want.end()));
    }

    let file = |s: &str| -> Result<u8, String> {
        if let Some(a) = names.file_by_name(s) {
            return Ok(a);
        }
        let v = number(s).ok_or_else(|| format!("'{s}' is not a register"))?;
        if v > 0xFF {
            return Err(format!("register 0x{v:X} is out of range"));
        }
        Ok((v & 0x7F) as u8)
    };
    let dest = |args: &[&str]| -> Result<bool, String> {
        match args.get(1).map(|s| s.to_ascii_lowercase()) {
            None => Ok(true),
            Some(s) if s == "f" || s == "1" => Ok(true),
            Some(s) if s == "w" || s == "0" => Ok(false),
            Some(s) => Err(format!("destination '{s}' should be w or f")),
        }
    };
    let bit = |s: &str| -> Result<u8, String> {
        let up = s.to_ascii_uppercase();
        if let Some((_, b)) = BITS.iter().find(|(n, _)| *n == up) {
            return Ok(*b);
        }
        match number(s) {
            Some(b) if b < 8 => Ok(b as u8),
            _ => Err(format!("'{s}' is not a bit 0-7")),
        }
    };
    let literal = |s: &str| -> Result<u8, String> {
        let v = number(s).ok_or_else(|| format!("'{s}' is not a number"))?;
        if v > 0xFF {
            return Err(format!("literal {v} does not fit in 8 bits"));
        }
        Ok(v as u8)
    };
    let address = |s: &str| -> Result<u16, String> {
        if let Some(a) = names.label_by_name(s) {
            return Ok(a);
        }
        let v = number(s).ok_or_else(|| format!("no label '{s}'"))?;
        if v as usize >= pic::ROM_WORDS {
            return Err(format!("address 0x{v:X} is past the end of program memory"));
        }
        Ok(v as u16)
    };

    let op = match mnem.as_str() {
        "ADDWF" => Addwf(file(args[0])?, dest(&args)?),
        "ANDWF" => Andwf(file(args[0])?, dest(&args)?),
        "COMF" => Comf(file(args[0])?, dest(&args)?),
        "DECF" => Decf(file(args[0])?, dest(&args)?),
        "DECFSZ" => Decfsz(file(args[0])?, dest(&args)?),
        "INCF" => Incf(file(args[0])?, dest(&args)?),
        "INCFSZ" => Incfsz(file(args[0])?, dest(&args)?),
        "IORWF" => Iorwf(file(args[0])?, dest(&args)?),
        "MOVF" => Movf(file(args[0])?, dest(&args)?),
        "RLF" => Rlf(file(args[0])?, dest(&args)?),
        "RRF" => Rrf(file(args[0])?, dest(&args)?),
        "SUBWF" => Subwf(file(args[0])?, dest(&args)?),
        "SWAPF" => Swapf(file(args[0])?, dest(&args)?),
        "XORWF" => Xorwf(file(args[0])?, dest(&args)?),
        "CLRF" => Clrf(file(args[0])?),
        "MOVWF" => Movwf(file(args[0])?),
        "CLRW" => Clrw,
        "NOP" => Nop,
        "BCF" => Bcf(file(args[0])?, bit(args[1])?),
        "BSF" => Bsf(file(args[0])?, bit(args[1])?),
        "BTFSC" => Btfsc(file(args[0])?, bit(args[1])?),
        "BTFSS" => Btfss(file(args[0])?, bit(args[1])?),
        "ADDLW" => Addlw(literal(args[0])?),
        "ANDLW" => Andlw(literal(args[0])?),
        "IORLW" => Iorlw(literal(args[0])?),
        "MOVLW" => Movlw(literal(args[0])?),
        "RETLW" => Retlw(literal(args[0])?),
        "SUBLW" => Sublw(literal(args[0])?),
        "XORLW" => Xorlw(literal(args[0])?),
        "CALL" => Call(address(args[0])?),
        "GOTO" => Goto(address(args[0])?),
        "CLRWDT" => Clrwdt,
        "RETFIE" => Retfie,
        "RETURN" => Return,
        "SLEEP" => Sleep,
        _ => unreachable!(),
    };
    Ok(encode(op))
}

/// Parse a number in any of the radix notations the assembler accepts.
pub fn number(s: &str) -> Option<u32> {
    let s = s.trim();
    let lower = s.to_ascii_lowercase();
    let quoted = |prefix: char| {
        lower
            .strip_prefix(prefix)
            .and_then(|r| r.strip_prefix('\''))
            .and_then(|r| r.strip_suffix('\''))
    };
    if let Some(h) = lower.strip_prefix("0x").or_else(|| quoted('h')) {
        return u32::from_str_radix(h, 16).ok();
    }
    if let Some(b) = lower.strip_prefix("0b").or_else(|| quoted('b')) {
        return u32::from_str_radix(b, 2).ok();
    }
    if let Some(d) = lower.strip_prefix('.').or_else(|| quoted('d')) {
        return d.parse().ok();
    }
    if let Some(h) = lower.strip_suffix('h')
        && h.starts_with(|c: char| c.is_ascii_digit())
    {
        return u32::from_str_radix(h, 16).ok();
    }
    lower.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_opcode_survives_decode_then_encode() {
        for w in 0..0x4000u16 {
            let op = decode(w);
            let back = encode(op);
            // Encodings with don't-care bits fold to their canonical form, which
            // must decode to the same instruction.
            assert_eq!(decode(back), op, "0x{w:04X}");
        }
    }

    #[test]
    fn assembles_with_names_and_radixes() {
        let mut names = Names::default();
        names.labels.insert(0x10, "loop".into());
        names.registers.insert(0x20, "count".into());
        let a = |s| assemble(s, &names).unwrap();
        assert_eq!(a("movlw 0x20"), 0x3020);
        assert_eq!(a("MOVLW h'FF'"), 0x30FF);
        assert_eq!(a("movlw b'1010'"), 0x300A);
        assert_eq!(a("movlw 10"), 0x300A);
        assert_eq!(a("movlw 0Ah"), 0x300A);
        assert_eq!(a("decfsz count,f"), 0x0BA0);
        assert_eq!(a("movf count,w"), 0x0820);
        assert_eq!(a("btfss STATUS,Z"), 0x1D03);
        assert_eq!(a("bsf STATUS,RP0"), 0x1683);
        assert_eq!(a("movwf TRISIO"), 0x0085);
        assert_eq!(a("goto loop"), 0x2810);
        assert_eq!(a("call 0x3FF ; osccal"), 0x23FF);
        assert!(assemble("movlw 300", &names).is_err());
        assert!(assemble("frob", &names).is_err());
    }

    #[test]
    fn disassembly_uses_names() {
        let mut names = Names::default();
        names.labels.insert(0x10, "loop".into());
        assert_eq!(disassemble(0x2810, &names, false), ("GOTO", "loop".into()));
        assert_eq!(
            disassemble(0x1D03, &names, false),
            ("BTFSS", "STATUS,Z".into())
        );
        assert_eq!(
            disassemble(0x0BA0, &names, false),
            ("DECFSZ", "0x20,f".into())
        );
        assert_eq!(disassemble(0x0085, &names, false), ("MOVWF", "GPIO".into()));
        assert_eq!(
            disassemble(0x0085, &names, true),
            ("MOVWF", "TRISIO".into())
        );
        assert_eq!(disassemble(0x0095, &names, false), ("MOVWF", "WPU".into()));
    }
}
