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

fn is_power_of_two(x: usize) -> bool {
    (x & (x - 1)) == 0
}

fn fft_inner(buf_a: &mut [Complex<f64>], buf_b: &mut [Complex<f64>], n: usize, step: usize) {
    const I: Complex<f64> = Complex { re: 0.0, im: 1.0 };

    if step >= n {
        return;
    }

    fft_inner(buf_b, buf_a, n, step * 2);
    fft_inner(&mut buf_b[step..], &mut buf_a[step..], n, step * 2);
    // create a slice for each half of buf_a:
    let (left, right) = buf_a.split_at_mut(n / 2);

    for i in (0..n).step_by(step * 2) {
        let t = (-I * PI * (i as f64) / (n as f64)).exp() * buf_b[i + step];
        left[i / 2] = buf_b[i] + t;
        right[i / 2] = buf_b[i] - t;
    }
}

pub fn fft(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    assert!(is_power_of_two(n));

    let mut buf_a: Vec<_> = signal.iter().map(Complex::from).collect();
    // alternate between buf_a and buf_b to avoid allocating a new vector each time:
    let mut buf_b = buf_a.clone();
    fft_inner(&mut buf_a, &mut buf_b, n, 1);
    buf_a.into_iter().map(Complex::norm).collect()
}
