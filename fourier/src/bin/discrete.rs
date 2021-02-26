use std::fs::File;
use std::io::Write;

use fourier::discrete_fourier;
use signal::gen_signal;

const HARMONICS: usize = 12;
const FREQUENCY: usize = 900;
const INTERVALS: usize = 256;
const DT: f64 = 1.0;

const USAGE: &str = "Usage: <signal.dat> <fourier.dat>";

fn save<T: Write>(mut dest: T, slice: &[f64], header: &str, mult: f64) {
    writeln!(dest, "{}", header).unwrap();
    for (i, s) in slice.iter().enumerate() {
        writeln!(dest, "{}\t{}", i as f64 * mult, s).unwrap();
    }
}

fn main() {
    let signal_dest = std::env::args().nth(1).expect(USAGE);
    let fourier_dest = std::env::args().nth(2).expect(USAGE);

    let signal_dest = File::create(signal_dest).unwrap();
    let fourier_dest = File::create(fourier_dest).unwrap();

    let signal = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    save(signal_dest, &signal, "# t\tx(t)", DT);

    let fourier = discrete_fourier(&signal);
    save(fourier_dest, &fourier, "# p\tX(p)", DT);
}
