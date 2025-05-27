#[cfg(feature = "profiling")]
use pprof::{protos::Message, ProfilerGuard};
#[cfg(feature = "profiling")]
use std::{
    fs::{self, File},
    path::Path,
};

use pi::generator;
use std::io::{self, Write};
mod pi;

const FLUSH_INTERVAL: u8 = 100u8;
const MAX_DIGITS: u128 = 5_000u128;

fn print_digits_of_pi() {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut g = generator();

    print!("3.");
    let _ = g.next();

    let mut flush_count = FLUSH_INTERVAL;
    let mut end_count = MAX_DIGITS;
    for digit in g {
        write!(out, "{}", digit).unwrap();
        if flush_count == 0 {
            flush_count = FLUSH_INTERVAL;
            out.flush().unwrap();
        } else {
            flush_count -= 1;
        }
        if end_count == 0 {
            break;
        } else {
            end_count -= 1;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "profiling")]
    let guard = ProfilerGuard::new(100).unwrap();

    print_digits_of_pi();

    #[cfg(feature = "profiling")]
    if let Ok(report) = guard.report().build() {
        let dir = Path::new("profiling");
        fs::create_dir_all(dir)?;

        let svg_path = dir.join("flamegraph.svg");
        report.flamegraph(File::create(svg_path)?)?;

        let pb_path = dir.join("profile.pb");
        report
            .pprof()?
            .write_to_writer(&mut File::create(pb_path)?)?;
    }

    Ok(())
}
