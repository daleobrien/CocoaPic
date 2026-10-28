//! Intel HEX, as MPASM and XC8 write it (INHX8M / INHX32).
//!
//! Addresses in the file are byte addresses, two per program word, little
//! endian. On the PIC12F675 program memory is 0x0000..0x07FF, the
//! configuration word is at word 0x2007 (byte 0x400E), and data EEPROM at
//! word 0x2100 (byte 0x4200) holds one EEPROM byte in the low half of each
//! word. On the PIC12C508 program memory is 0x0000..0x03FF and the
//! configuration word is at word 0xFFF (byte 0x1FFE).
//!
//! A hex file does not say which part it is for: it loads into the part the
//! [`Pic`] already is.

use anyhow::{Context, Result, bail};

use crate::isa::Core;
use crate::pic::{Config, EEPROM_BYTES, Pic};

const EEPROM_WORD: u32 = 0x2100;

/// Where the configuration word lives.
fn config_word(core: Core) -> u32 {
    match core {
        Core::Midrange => 0x2007,
        Core::Baseline => 0xFFF,
    }
}

/// Load a hex image into the part. Program words absent from the file are left
/// erased; the configuration word and EEPROM keep their defaults if absent.
pub fn load(pic: &mut Pic, text: &str) -> Result<()> {
    let mut bytes: Vec<(u32, u8)> = Vec::new();
    let mut upper = 0u32;
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let rec = line
            .strip_prefix(':')
            .with_context(|| format!("line {}: no ':' record mark", n + 1))?;
        let raw = (0..rec.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(rec.get(i..i + 2).unwrap_or("?"), 16))
            .collect::<Result<Vec<u8>, _>>()
            .with_context(|| format!("line {}: not hex", n + 1))?;
        if raw.len() < 5 || raw.len() != raw[0] as usize + 5 {
            bail!("line {}: record length does not match", n + 1);
        }
        if raw.iter().fold(0u8, |a, b| a.wrapping_add(*b)) != 0 {
            bail!("line {}: bad checksum", n + 1);
        }
        let addr = u16::from_be_bytes([raw[1], raw[2]]) as u32;
        let data = &raw[4..raw.len() - 1];
        match raw[3] {
            0x00 => {
                for (i, b) in data.iter().enumerate() {
                    bytes.push((upper + addr + i as u32, *b));
                }
            }
            0x01 => break,
            0x04 if data.len() == 2 => {
                upper = (u16::from_be_bytes([data[0], data[1]]) as u32) << 16
            }
            0x02 if data.len() == 2 => upper = (u16::from_be_bytes([data[0], data[1]]) as u32) << 4,
            0x03 | 0x05 => {}
            t => bail!("line {}: unsupported record type {t:02X}", n + 1),
        }
    }

    pic.erase();
    pic.config = pic.part.default_config();
    pic.eeprom = [0xFF; EEPROM_BYTES];
    let core = pic.part.core();
    let config_word = config_word(core);
    let high_bits = (core.word_mask() >> 8) as u8;
    for (addr, b) in bytes {
        let word = addr / 2;
        let high = addr & 1 == 1;
        let set = |w: &mut u16| {
            *w = if high {
                (*w & 0x00FF) | ((b & high_bits) as u16) << 8
            } else {
                (*w & 0xFF00) | b as u16
            }
        };
        match word {
            w if (w as usize) < pic.rom.len() => set(&mut pic.rom[w as usize]),
            w if w == config_word => {
                let mut c = pic.config.0;
                set(&mut c);
                pic.config = Config(c);
            }
            w if core == Core::Midrange
                && (EEPROM_WORD..EEPROM_WORD + EEPROM_BYTES as u32).contains(&w)
                && !high =>
            {
                pic.eeprom[(w - EEPROM_WORD) as usize] = b;
            }
            _ => {}
        }
    }
    Ok(())
}

/// Write the part as INHX32: program memory up to the last unerased word, the
/// configuration word, and EEPROM if it holds anything but 0xFF.
pub fn save(pic: &Pic) -> String {
    let mut out = String::new();
    let used = pic
        .rom
        .iter()
        .rposition(|w| *w != pic.part.erased())
        .map_or(0, |i| i + 1);
    let mut prog = Vec::new();
    for w in &pic.rom[..used] {
        prog.extend_from_slice(&w.to_le_bytes());
    }
    record(&mut out, 0x04, 0, &[0, 0]);
    for (i, chunk) in prog.chunks(16).enumerate() {
        record(&mut out, 0x00, (i * 16) as u16, chunk);
    }
    if pic.eeprom.iter().any(|b| *b != 0xFF) {
        let mut ee = Vec::new();
        for b in pic.eeprom {
            ee.extend_from_slice(&[b, 0]);
        }
        for (i, chunk) in ee.chunks(16).enumerate() {
            record(
                &mut out,
                0x00,
                (EEPROM_WORD * 2) as u16 + (i * 16) as u16,
                chunk,
            );
        }
    }
    record(
        &mut out,
        0x00,
        (config_word(pic.part.core()) * 2) as u16,
        &pic.config.0.to_le_bytes(),
    );
    record(&mut out, 0x01, 0, &[]);
    out
}

fn record(out: &mut String, kind: u8, addr: u16, data: &[u8]) {
    let mut raw = vec![data.len() as u8];
    raw.extend_from_slice(&addr.to_be_bytes());
    raw.push(kind);
    raw.extend_from_slice(data);
    let sum = raw
        .iter()
        .fold(0u8, |a, b| a.wrapping_add(*b))
        .wrapping_neg();
    raw.push(sum);
    out.push(':');
    for b in raw {
        out.push_str(&format!("{b:02X}"));
    }
    out.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_program_config_and_eeprom() {
        let mut pic = Pic::new();
        pic.rom[0] = 0x3020;
        pic.rom[1] = 0x0085;
        pic.rom[0x20] = 0x2800;
        pic.config = Config(0x3184);
        pic.eeprom[3] = 0x42;
        let text = save(&pic);
        let mut back = Pic::new();
        load(&mut back, &text).unwrap();
        assert_eq!(back.rom, pic.rom);
        assert_eq!(back.config, pic.config);
        assert_eq!(back.eeprom, pic.eeprom);
    }

    #[test]
    fn reads_an_mpasm_file() {
        // movlw 0x07 / movwf CMCON / goto $ — and a config word.
        let text = ":020000040000FA\n:0600000007309900022800\n:02400E00843FED\n:00000001FF\n";
        let mut pic = Pic::new();
        load(&mut pic, text).unwrap();
        assert_eq!(&pic.rom[..3], &[0x3007, 0x0099, 0x2802]);
        assert_eq!(pic.config, Config(0x3F84));
    }

    #[test]
    fn round_trips_a_12c508() {
        use crate::pic::Part;
        let mut pic = Pic::with_part(Part::Pic12C508);
        pic.rom[0] = 0x025;
        pic.rom[1] = 0xC3E;
        pic.rom[2] = 0x006;
        pic.config = Config(0xFEE);
        let text = save(&pic);
        assert!(
            text.contains(":021FFE00EE0FE4"),
            "config at word 0xFFF:\n{text}"
        );
        let mut back = Pic::with_part(Part::Pic12C508);
        load(&mut back, &text).unwrap();
        assert_eq!(back.rom, pic.rom);
        assert_eq!(back.config, pic.config);
        assert_eq!(back.rom[0x1FF], 0xC70, "the calibration word survives");
    }
}
