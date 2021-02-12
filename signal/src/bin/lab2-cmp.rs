use signal::{correlation, gen_signal};
use std::time::{Duration, Instant};

const HARMONICS: usize = 1024;
const FREQUENCY: usize = 1024;
const INTERVALS: usize = 1 << 19;
const DT: f64 = 0.01;

fn test(a: &[f64], b: &[f64]) -> Duration {
    let timer = Instant::now();
    let corr = correlation(a, b);
    fn id<T>(x: T) -> T {
        x
    }
    assert_eq!(corr, id(corr)); // To prevent optimisations
    timer.elapsed()
}

fn main() {
    let sig1 = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    let sig2 = gen_signal(HARMONICS, FREQUENCY, INTERVALS, DT);
    let time1 = test(&sig1, &sig2);
    let time2 = test(&sig1, &sig1);
    println!("Кореляція: {:?}", time1);
    println!("Автокореляція: {:?}", time2);
}
