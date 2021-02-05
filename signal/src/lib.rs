use rand::Rng;

pub fn gen_signal(harmonics: usize, frequency: usize, intervals: usize, dt: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut res = vec![0.0; intervals];
    let dw = frequency as f64 / harmonics as f64;
    let mut w = dw;
    for _ in 0..harmonics {
        let a: f64 = rng.gen();
        let phi: f64 = rng.gen();
        for t in 0..intervals {
            let x = a * (w * t as f64 * dt + phi).sin();
            res[t] += x;
        }
        w += dw;
    }
    res
}

pub fn average(arr: &[f64]) -> f64 {
    arr.iter().sum::<f64>() / arr.len() as f64
}

// Returns (average, disspersion) of slice
pub fn stats(arr: &[f64]) -> (f64, f64) {
    let avg = average(arr);
    let diss = arr.iter().map(|x| (avg - x).powi(2)).sum::<f64>() / arr.len() as f64;
    (avg, diss)
}

pub fn correlation(a: &[f64], b: &[f64]) -> f64 {
    let ma = average(a);
    let mb = average(b);
    let mut num = 0.0;
    let mut sqr_a = 0.0;
    let mut sqr_b = 0.0;
    for (a, b) in a.iter().zip(b.iter()) {
        num += (a - ma) * (b - mb);
        sqr_a += (a - ma).powi(2);
        sqr_b += (b - mb).powi(2);
    }
    num / (sqr_a * sqr_b).sqrt()
}
