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
