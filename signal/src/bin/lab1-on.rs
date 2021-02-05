use signal::gen_signal;
use std::time::Instant;

const HARMONICS_LO: usize = 10;
const HARMONICS_HI: usize = 2_000_000;
const HARMONICS_DELTA: usize = 2;

const FREQUENCY: usize = 900;
const INTERVALS: usize = 256;

fn test(harmonics: usize) -> u128 {
    let timer = Instant::now();
    gen_signal(harmonics, FREQUENCY, INTERVALS, 1.0);
    timer.elapsed().as_millis()
}

fn main() {
    println!("# n\tO(n)");
    let mut harmonics = HARMONICS_LO;
    while harmonics <= HARMONICS_HI {
        let duration = test(harmonics);
        println!("{}\t{}", harmonics, duration);
        harmonics *= HARMONICS_DELTA;
    }
}
