use core::f64;
use fourier::fft;
use num::Complex;
use rustfft::FftPlanner;
use signal::gen_signal;
use std::io::Write;
use std::{fmt::Display, fs::File};

const HARMONICS: usize = 12;
const FREQUENCY: usize = 900;
const INTERVALS: usize = 256;
const DT: f64 = 1.0;

const USAGE: &str = "Usage: <signal.dat> <fourier.dat>";

fn save<T: Write, D: Display>(mut dest: T, slice: &[D], header: &str, mult: f64) {
    writeln!(dest, "{}", header).unwrap();
    for (i, s) in slice.iter().enumerate() {
        writeln!(dest, "{}\t{}", i as f64 * mult, s).unwrap();
    }
}

fn lib_fft(signal: &[f64]) -> Vec<f64> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(signal.len());

    let mut buffer: Vec<_> = signal.iter().map(Complex::from).collect();
    fft.process(&mut buffer);
    buffer.into_iter().map(Complex::norm).collect()
}

fn almost_eq(a: &[f64], b: &[f64]) -> bool {
    const EPS: f64 = 0.0000000001;
    if a.len() != b.len() {
        return false;
    }
    !a.iter().zip(b.iter()).any(|(a, b)| (a - b).abs() > EPS)
}

fn main() {
    let signal_dest = std::env::args().nth(1).expect(USAGE);
    let fourier_dest = std::env::args().nth(2).expect(USAGE);

    let signal_dest = File::create(signal_dest).unwrap();
    let fourier_dest = File::create(fourier_dest).unwrap();

    let signal = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    save(signal_dest, &signal, "# t\tx(t)", DT);

    let fourier = fft(&signal);
    save(fourier_dest, &fourier, "# p\tX(p)", DT);

    // Compare with library
    let lib = lib_fft(&signal);
    assert!(almost_eq(&lib, &fourier));
}
