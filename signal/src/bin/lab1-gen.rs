use signal::{gen_signal, stats};

const HARMONICS: usize = 12;
const FREQUENCY: usize = 900;
const INTERVALS: usize = 256;
const DT: f64 = 1.0;

fn main() {
    let res = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    println!("# t\tx(t)");
    for (t, x) in res.iter().enumerate() {
        let t = t as f64 * DT;
        println!("{}\t{}", t, x);
    }
    let (avg, disp) = stats(&res);
    eprintln!("average: {}", avg);
    eprintln!("dispersion: {}", disp);
}
