use core::f64;
use fourier::{discrete_fourier, fft};
use signal::gen_signal;
use std::{fs::File, io::Write};

const HARMONICS: usize = 12;
const FREQUENCY: usize = 900;
const INTERVALS: usize = 256;
const DT: f64 = 1.0;

const USAGE: &str = "Usage: <diff.dat>";

fn save<T: Write>(mut dest: T, slice: &[f64], header: &str, mult: f64) {
    writeln!(dest, "{}", header).unwrap();
    for (i, s) in slice.iter().enumerate() {
        writeln!(dest, "{}\t{}", i as f64 * mult, s).unwrap();
    }
}
fn main() {
    let diff_dest = std::env::args().nth(1).expect(USAGE);
    let diff_dest = File::create(diff_dest).unwrap();

    let signal = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    let mut fft = fft(&signal);
    let dft = discrete_fourier(&signal);

    for (a, b) in fft.iter_mut().zip(dft.iter()) {
        *a -= *b;
    }
    save(diff_dest, &fft, "# x\tdiff(x)", DT);
}
