use num::Complex;
use std::f64::consts::PI;

fn w_coeff(p: usize, k: usize, n: usize) -> Complex<f64> {
    let (p, k, n) = (p as f64, k as f64, n as f64);
    let x = 2.0 * PI / n * p * k;
    Complex::new(x.cos(), 0.0) - Complex::new(0.0, -x.sin())
}

pub struct Wtable {
    table: Vec<Complex<f64>>,
    n: usize,
}

impl Wtable {
    fn new(n: usize) -> Self {
        let mut table = Vec::with_capacity(n * n);
        for i in 0..n {
            for j in 0..n {
                table.push(w_coeff(i, j, n));
            }
        }
        Self { table, n }
    }

    fn get(&self, p: usize, k: usize) -> Complex<f64> {
        let index = p * self.n + k;
        self.table[index]
    }
}

pub fn discrete_fourier(signal: &[f64]) -> Vec<f64> {
    let wcoeffs = Wtable::new(signal.len());
    (0..signal.len())
        .map(|p| {
            signal
                .iter()
                .cloned()
                .enumerate()
                .map(|(k, x)| x * wcoeffs.get(p, k))
                .sum::<Complex<f64>>()
        })
        .map(Complex::norm)
        .collect()
}
