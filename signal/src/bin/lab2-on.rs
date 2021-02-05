use signal::{correlation, gen_signal};
use std::time::Instant;

const HARMONICS: usize = 12;
const FREQUENCY: usize = 900;
const INTERVALS_LO: usize = 256;
const INTERVALS_HI: usize = 2 << 20;
const INTERVALS_DELTA: usize = 2;

fn id<T>(x: T) -> T { x }

fn test(arr: &[f64], size: usize) -> u128 {
    let timer = Instant::now();
    let a = &arr[..size];
    let b = &arr[..size];
    let corr = correlation(a, b);
    assert_eq!(corr, id(corr)); // To prevent optimisations
    timer.elapsed().as_millis()
}

fn main() {
    println!("# n\tO(n)");
    let sig = gen_signal(HARMONICS, FREQUENCY, INTERVALS_HI, 1.0);
    let mut intervals = INTERVALS_LO;
    while intervals <= INTERVALS_HI {
        let duration = test(&sig, intervals);
        println!("{}\t{}", intervals, duration);
        intervals *= INTERVALS_DELTA;
    }
}
