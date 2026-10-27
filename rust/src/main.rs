//! CocoaPic: a PIC12F675 simulator — the native macOS window, and a CLI over
//! the same core for running and inspecting programs from a terminal.

mod hex;
mod isa;
mod pic;
mod project;

#[cfg(target_os = "macos")]
mod gui;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::pic::Pic;

#[derive(Parser)]
#[command(name = "cocoapic", version, about = "PIC12F675 simulator")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Open the native window (the default), optionally on a program.
    Gui { file: Option<PathBuf> },
    /// Run a program for a number of cycles and print the machine state.
    Run {
        file: PathBuf,
        #[arg(short, long, default_value_t = 1_000_000)]
        cycles: u64,
    },
    /// Print a program's listing.
    Disasm { file: PathBuf },
    /// Assemble a source file to Intel HEX.
    Asm {
        file: PathBuf,
        #[arg(short, long)]
        out: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        None => gui(None),
        Some(Command::Gui { file }) => gui(file),
        Some(Command::Run { file, cycles }) => {
            let mut pic = Pic::new();
            let ann = project::open(&file, &mut pic)?;
            let started = std::time::Instant::now();
            let stop = pic.run(cycles, &|| false);
            let secs = started.elapsed().as_secs_f64();
            println!("stopped: {stop:?} after {} cycles", pic.cycles);
            println!(
                "speed:   {:.1} MIPS ({:.1}× real time)",
                pic.cycles as f64 / secs / 1e6,
                pic.cycles as f64 / secs / pic::CYCLES_PER_SECOND as f64
            );
            println!(
                "PC 0x{:03X}  W 0x{:02X}  STATUS 0x{:02X}  stack {:?}",
                pic.pc,
                pic.w,
                pic.status(),
                pic.stack_view()
            );
            print!("GPIO ");
            for p in (0..6).rev() {
                print!("{}", if pic.pin_level(p) { '1' } else { '0' });
            }
            println!();
            for a in 0x20..0x60u8 {
                if let Some(n) = ann.names.registers.get(&a) {
                    println!("{n:>10} [0x{a:02X}] = 0x{:02X}", pic.peek(a));
                }
            }
            Ok(())
        }
        Some(Command::Disasm { file }) => {
            let mut pic = Pic::new();
            let ann = project::open(&file, &mut pic)?;
            let banks = isa::banks(&pic.rom);
            for (addr, word) in pic.rom.iter().enumerate() {
                if *word == pic::ERASED {
                    continue;
                }
                let (m, ops) = isa::disassemble(*word, &ann.names, banks[addr]);
                let label = ann.names.label(addr as u16).unwrap_or("");
                let note = ann.notes.get(&(addr as u16)).map(|n| format!("; {n}"));
                println!(
                    "{addr:03X}  {word:04X}  {label:<10} {m:<7} {ops:<16} {}",
                    note.unwrap_or_default()
                );
            }
            Ok(())
        }
        Some(Command::Asm { file, out }) => {
            let mut pic = Pic::new();
            let ann = project::assemble_source(&std::fs::read_to_string(&file)?, &mut pic)?;
            let out = out.unwrap_or_else(|| file.with_extension("hex"));
            let written = project::save(&out, &pic, &ann)?;
            println!("wrote {}", written.display());
            Ok(())
        }
    }
}

#[cfg(target_os = "macos")]
fn gui(file: Option<PathBuf>) -> Result<()> {
    gui::run(file)
}

#[cfg(not(target_os = "macos"))]
fn gui(_file: Option<PathBuf>) -> Result<()> {
    anyhow::bail!("the window is macOS only; use `run`, `disasm` or `asm`")
}
