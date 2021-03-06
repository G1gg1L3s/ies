use signal::{correlation, gen_signal};

const HARMONICS: usize = 12;
const FREQUENCY: usize = 900;
const INTERVALS: usize = 256;
const DT: f64 = 0.001;

fn main() {
    let sig = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    let mid = sig.len() / 2;
    let a = &sig[..mid];
    println!("# tau\tcorrelation");
    for tau in 0..mid {
        let b = &sig[tau..tau + mid];
        let corr = correlation(a, b);
        println!("{}\t{}", tau, corr);
    }
}
