//! How fast the simulator core runs with Realtime off: `make bench`.
//!
//! Each program is loaded once, then every iteration clones the machine and
//! runs it for `CYCLES` instruction cycles, as the window's full-speed run
//! does. Criterion's throughput line ("elem/s") is simulated cycles per
//! second, so `100 Melem/s` reads as 100 MIPS, or 100× real time at 4 MHz.

use std::path::Path;

use cocoapic::pic::{Pic, Stop};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};

const CYCLES: u64 = 1_000_000;

/// The programs: the window's blink demo (a tight wait on T0IF), the counter
/// (asleep, waiting on a pin), and a busy loop of mixed instructions.
const PROGRAMS: &[(&str, &str)] = &[
    ("blink", "examples/blink.asm"),
    ("counter", "examples/counter.asm"),
    ("mixed", "benches/mixed.asm"),
];

fn load(file: &str) -> Pic {
    let mut pic = Pic::new();
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(file);
    cocoapic::project::open(&path, &mut pic).expect("benchmark program loads");
    pic
}

fn full_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_speed");
    group.throughput(Throughput::Elements(CYCLES));
    for (name, file) in PROGRAMS {
        let pic = load(file);
        group.bench_function(*name, |b| {
            b.iter_batched_ref(
                || pic.clone(),
                |pic| {
                    let stop = pic.run(CYCLES, &|| false);
                    assert_eq!(stop, Stop::Budget);
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, full_speed);
criterion_main!(benches);
