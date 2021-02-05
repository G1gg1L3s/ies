use signal::{correlation, gen_signal};

const HARMONICS: usize = 12;
const FREQUENCY: usize = 900;
const INTERVALS: usize = 256;
const DT: f64 = 0.001;

fn main() {
    let sig = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    println!("# tau\tcorrelation");
    for tau in 0..INTERVALS / 2 {
        let a = &sig[tau..];
        let b = &sig[..a.len()];
        let corr = correlation(a, b);
        println!("{}\t{}", tau, corr);
    }
}
