//! A project: the part's program and EEPROM plus the things only a person
//! adds — labels, register names, notes on lines, breakpoints.
//!
//! The program travels as Intel HEX so any programmer can burn it; the rest
//! lives in a JSON sidecar beside it (`blink.hex` → `blink.cocoapic.json`),
//! which opening the hex picks up again. A source file (`.asm`) can be
//! opened too: [`assemble_source`] is a small two-pass assembler — labels,
//! `equ`, `org`, `__config`, comments — enough for the listings the old app
//! was used with, not a replacement for MPASM.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::hex;
use crate::isa::{self, Names};
use crate::pic::{Config, Pic, ROM_WORDS};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Annotations {
    #[serde(default)]
    pub names: Names,
    #[serde(default)]
    pub notes: BTreeMap<u16, String>,
    #[serde(default)]
    pub breakpoints: Vec<u16>,
}

impl Annotations {
    pub fn apply_breakpoints(&self, pic: &mut Pic) {
        pic.breakpoints = vec![false; ROM_WORDS];
        for &b in &self.breakpoints {
            if let Some(slot) = pic.breakpoints.get_mut(b as usize) {
                *slot = true;
            }
        }
    }
}

/// Where the sidecar for a program file lives.
pub fn sidecar(path: &Path) -> PathBuf {
    path.with_extension("cocoapic.json")
}

/// Open a `.hex` (with its sidecar, if any) or an `.asm` source into `pic`.
pub fn open(path: &Path, pic: &mut Pic) -> Result<Annotations> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let is_source = path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| matches!(e.to_ascii_lowercase().as_str(), "asm" | "s" | "src"));
    let ann = if is_source {
        assemble_source(&text, pic)?
    } else {
        hex::load(pic, &text)?;
        match std::fs::read_to_string(sidecar(path)) {
            Ok(json) => serde_json::from_str(&json).context("reading the sidecar")?,
            Err(_) => Annotations::default(),
        }
    };
    ann.apply_breakpoints(pic);
    pic.reset();
    Ok(ann)
}

/// Save the program as hex and the annotations beside it. Returns the hex path
/// actually written: a source file is saved as hex next to itself.
pub fn save(path: &Path, pic: &Pic, ann: &Annotations) -> Result<PathBuf> {
    let path = path.with_extension("hex");
    std::fs::write(&path, hex::save(pic)).with_context(|| format!("writing {}", path.display()))?;
    let mut ann = ann.clone();
    ann.breakpoints = pic
        .breakpoints
        .iter()
        .enumerate()
        .filter(|(_, b)| **b)
        .map(|(i, _)| i as u16)
        .collect();
    std::fs::write(sidecar(&path), serde_json::to_string_pretty(&ann)?)?;
    Ok(path)
}

/// Assemble a whole source file into `pic`, returning the labels, `equ` names
/// and end-of-line comments it carried.
pub fn assemble_source(src: &str, pic: &mut Pic) -> Result<Annotations> {
    let mut ann = Annotations::default();
    let mut lines = Vec::new();

    // Pass 1: lay out addresses, collect labels and names.
    let mut pc: u32 = 0;
    for (n, raw) in src.lines().enumerate() {
        let (code, comment) = match raw.find(';') {
            Some(i) => (&raw[..i], raw[i + 1..].trim()),
            None => (raw, ""),
        };
        let starts_in_column_one = !code.starts_with(char::is_whitespace);
        let mut words: Vec<&str> = code.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        // A label is anything in column one (with or without a colon) that is
        // not itself a directive or an instruction.
        let first = words[0].trim_end_matches(':');
        let is_label = words[0].ends_with(':')
            || (starts_in_column_one
                && !is_keyword(first)
                && words.get(1).map(|w| w.to_ascii_lowercase()) != Some("equ".into()));
        if words.len() >= 3 && words[1].eq_ignore_ascii_case("equ") {
            let v = isa::number(words[2])
                .with_context(|| format!("line {}: bad value for {}", n + 1, words[0]))?;
            if v < 0x100 {
                ann.names
                    .registers
                    .insert((v & 0x7F) as u8, words[0].to_string());
            }
            continue;
        }
        if is_label {
            ann.names.labels.insert(pc as u16, first.to_string());
            words.remove(0);
            if words.is_empty() {
                continue;
            }
        }
        match words[0].to_ascii_lowercase().as_str() {
            "org" => {
                pc = words
                    .get(1)
                    .and_then(|s| isa::number(s))
                    .with_context(|| format!("line {}: org needs an address", n + 1))?;
            }
            "__config" => {
                let v = words
                    .get(1)
                    .and_then(|s| isa::number(s))
                    .with_context(|| format!("line {}: __config needs a value", n + 1))?;
                pic.config = Config(v as u16 & 0x3FFF);
            }
            "list" | "include" | "#include" | "radix" | "processor" | "errorlevel" | "end" => {}
            _ => {
                if pc as usize >= ROM_WORDS {
                    bail!("line {}: past the end of program memory", n + 1);
                }
                if !comment.is_empty() {
                    ann.notes.insert(pc as u16, comment.to_string());
                }
                lines.push((n + 1, pc as u16, words.join(" ")));
                pc += 1;
            }
        }
    }

    // Pass 2: every label is known, so forward references resolve.
    pic.erase();
    for (line, addr, text) in lines {
        let text = text.replace('$', &addr.to_string());
        pic.rom[addr as usize] =
            isa::assemble(&text, &ann.names).map_err(|e| anyhow::anyhow!("line {line}: {e}"))?;
    }
    Ok(ann)
}

fn is_keyword(word: &str) -> bool {
    let w = word.to_ascii_lowercase();
    matches!(
        w.as_str(),
        "org"
            | "__config"
            | "list"
            | "include"
            | "#include"
            | "radix"
            | "processor"
            | "errorlevel"
            | "end"
    ) || isa::assemble(&format!("{w} 0"), &Names::default()).is_ok()
        || isa::assemble(&w, &Names::default()).is_ok()
        || isa::assemble(&format!("{w} 0,0"), &Names::default()).is_ok()
}

/// The program the window opens with: GP0 toggles each time TMR0 overflows,
/// so an LED there blinks and the pin view shows it.
pub const DEMO: &str = "\
        __config 0x3F94     ; INTOSC, WDT off, MCLR off
count   equ 0x20

        org 0
start   movlw 7
        movwf CMCON         ; comparator off: GP0/GP1 digital
        bsf STATUS,RP0      ; bank 1
        clrf ANSEL          ; no analog inputs
        movlw 0x3E
        movwf TRISIO        ; GP0 is an output
        movlw 0x87
        movwf OPTION_REG    ; TMR0 from Fosc/4, prescaler 1:256
        bcf STATUS,RP0      ; bank 0
        clrf GPIO
        clrf count
loop    btfss INTCON,T0IF   ; wait for TMR0 to overflow
        goto loop
        bcf INTCON,T0IF
        movlw 1
        xorwf GPIO,f        ; toggle GP0
        incf count,f
        goto loop
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_demo_assembles_and_blinks() {
        let mut pic = Pic::new();
        let ann = assemble_source(DEMO, &mut pic).unwrap();
        assert_eq!(ann.names.labels.get(&11).map(String::as_str), Some("loop"));
        assert_eq!(
            ann.names.registers.get(&0x20).map(String::as_str),
            Some("count")
        );
        assert!(!pic.config.wdt());
        pic.reset();
        // 256 × 256 cycles per overflow; three overflows toggle GP0 three times.
        pic.run(3 * 65536 + 200, &|| false);
        assert_eq!(pic.peek(0x20), 3);
        assert!(pic.pin_level(0));
    }
}
