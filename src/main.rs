//! CocoaPic: a PIC12F675 and PIC12C508 simulator — the native macOS window,
//! and a CLI over the same core for running and inspecting programs from a
//! terminal.

#[cfg(target_os = "macos")]
mod gui;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use cocoapic::pic::{Part, Pic};
use cocoapic::{isa, pic, project};

#[derive(Parser)]
#[command(
    name = "cocoapic",
    version,
    about = "PIC12F675 and PIC12C508 simulator"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    /// The part, 12F675 or 12C508, for a program that does not say: a hex
    /// file without its sidecar, source without a `processor` line, or the
    /// window's demo.
    #[arg(long, global = true, value_parser = parse_part, default_value = "12F675")]
    part: Part,
}

fn parse_part(s: &str) -> Result<Part, String> {
    Part::from_name(s).ok_or_else(|| format!("no part '{s}': try 12F675 or 12C508"))
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
    let part = cli.part;
    match cli.command {
        None => gui(None, part),
        Some(Command::Gui { file }) => gui(file, part),
        Some(Command::Run { file, cycles }) => {
            let mut pic = Pic::with_part(part);
            let ann = project::open(&file, &mut pic)?;
            let started = std::time::Instant::now();
            let stop = pic.run(cycles, || false);
            let secs = started.elapsed().as_secs_f64();
            println!("stopped: {stop:?} after {} cycles", pic.cycles);
            println!(
                "speed:   {:.1} MIPS ({:.1}× real time)",
                pic.cycles as f64 / secs / 1e6,
                pic.cycles as f64 / secs / pic::CYCLES_PER_SECOND as f64
            );
            println!(
                "{}  PC 0x{:03X}  W 0x{:02X}  STATUS 0x{:02X}  stack {:?}",
                pic.part.name(),
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
            for a in pic.part.gprs() {
                if let Some(n) = ann.names.registers.get(&a) {
                    println!("{n:>10} [0x{a:02X}] = 0x{:02X}", pic.peek(a));
                }
            }
            Ok(())
        }
        Some(Command::Disasm { file }) => {
            let mut pic = Pic::with_part(part);
            let ann = project::open(&file, &mut pic)?;
            let banks = isa::banks(&pic.rom, pic.part);
            for (addr, word) in pic.rom.iter().enumerate() {
                if *word == pic.part.erased() {
                    continue;
                }
                let (m, ops) = isa::disassemble(*word, &ann.names, banks[addr], pic.part);
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
            let mut pic = Pic::with_part(part);
            let ann = project::assemble_source(&std::fs::read_to_string(&file)?, &mut pic)?;
            let out = out.unwrap_or_else(|| file.with_extension("hex"));
            let written = project::save(&out, &pic, &ann)?;
            println!("wrote {}", written.display());
            Ok(())
        }
    }
}

#[cfg(target_os = "macos")]
fn gui(file: Option<PathBuf>, part: Part) -> Result<()> {
    gui::run(file, part)
}

#[cfg(not(target_os = "macos"))]
fn gui(_file: Option<PathBuf>, _part: Part) -> Result<()> {
    anyhow::bail!("the window is macOS only; use `run`, `disasm` or `asm`")
}

#[cfg(test)]
mod rt_bench {
    use cocoapic::pic::{self, Pic, Stop};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    fn measure(file: &str, secs: f64) {
        let mut p = Pic::new();
        cocoapic::project::open(std::path::Path::new(file), &mut p).unwrap();
        let pic = Arc::new(Mutex::new(p));
        let stop = Arc::new(AtomicBool::new(false));
        let t = {
            let pic = pic.clone();
            let stop = stop.clone();
            std::thread::spawn(move || {
                let began = Instant::now();
                let start_cycles = 0u64;
                let mut late_max = Duration::ZERO;
                loop {
                    let (reason, cycles) = {
                        let mut pic = pic.lock().unwrap();
                        let r = pic.run(1_000, || stop.load(Ordering::Relaxed));
                        (r, pic.cycles)
                    };
                    if reason != Stop::Budget || stop.load(Ordering::Relaxed) {
                        break;
                    }
                    let due = Duration::from_secs_f64(
                        (cycles - start_cycles) as f64 / pic::CYCLES_PER_SECOND as f64,
                    );
                    match due.checked_sub(began.elapsed()) {
                        Some(w) => std::thread::sleep(w),
                        None => late_max = late_max.max(began.elapsed() - due),
                    }
                }
                late_max
            })
        };
        // The "GUI": clone the machine every 33 ms, as poll()/refresh() do.
        let began = Instant::now();
        let mut clone_max = Duration::ZERO;
        let mut samples = vec![];
        while began.elapsed().as_secs_f64() < secs {
            std::thread::sleep(Duration::from_millis(33));
            let t0 = Instant::now();
            let copy = pic.lock().unwrap().clone();
            clone_max = clone_max.max(t0.elapsed());
            samples.push((began.elapsed().as_secs_f64(), copy.cycles));
        }
        stop.store(true, Ordering::Relaxed);
        let late = t.join().unwrap();
        let (el, cyc) = *samples.last().unwrap();
        let rate = cyc as f64 / el;
        // worst deviation of any 1 s window
        let mut worst: f64 = 0.0;
        for i in 0..samples.len() {
            for j in i..samples.len() {
                let dt = samples[j].0 - samples[i].0;
                if (dt - 1.0).abs() < 0.02 {
                    let r = (samples[j].1 - samples[i].1) as f64 / dt;
                    worst = worst.max((r / 1e6 - 1.0).abs());
                }
            }
        }
        println!(
            "{file}: {:.4} M cycles/s over {el:.2}s ({:+.3}% vs 1 MHz), worst 1s window off by {:.3}%, max clone/lock wait {:?}, max lateness {:?}",
            rate / 1e6,
            (rate / 1e6 - 1.0) * 100.0,
            worst * 100.0,
            clone_max,
            late
        );
    }

    /// Real-time pacing check: `cargo test --release realtime_rate -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn realtime_rate() {
        measure("examples/blink.asm", 10.0);
        measure("examples/counter.asm", 10.0);
    }
}
