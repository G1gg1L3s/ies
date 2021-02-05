use signal::gen_signal;

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
    let avg = res.iter().sum::<f64>() / res.len() as f64;
    let disp = res
        .iter()
        .map(|x| {
            let x = avg - x;
            x * x
        })
        .sum::<f64>()
        / (res.len() - 1) as f64;
    eprintln!("average: {}", avg);
    eprintln!("dispersion: {}", disp);
}
