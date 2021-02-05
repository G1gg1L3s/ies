use signal::{correlation, gen_signal};

const HARMONICS: usize = 12;
const FREQUENCY: usize = 900;
const INTERVALS: usize = 256;
const DT: f64 = 0.01;

fn main() {
    let sig1 = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    let sig2 = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    let mid = sig1.len() / 2;
    let a = &sig1[..mid];
    println!("# tau\tcorrelation");
    for tau in 0..mid {
        let b = &sig2[tau..tau + mid];
        let corr = correlation(a, b);
        println!("{}\t{}", tau, corr);
    }
}
