use prob_theory_lab1::{c, factorial};
use rand::prelude::*;

fn bernulli(n: u32, p: f32, rng: &mut ThreadRng) -> usize {
    (0..n).map(|_| rng.gen::<f32>()).filter(|&x| x < p).count()
}

fn exact_benulli(n: u32, p: f64, k: u32) -> f64 {
    c(n, k) * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32)
}

fn approx_bernulli(n: u32, p: f64, k: u32) -> f64 {
    let lambda = n as f64 * p;
    f64::exp(-lambda) * lambda.powi(k as i32) / factorial(k)
}

fn main() {
    let iters = 10000;
    let mut rng = rand::thread_rng();

    for n in [10, 100, 500] {
        for p in [0.001, 0.01, 0.1, 0.25, 0.5] {
            let mid = n as f32 / 2.0;
            let shift = (n as f32 * p * (1.0 - p)).sqrt();

            let success = (0..iters)
                .map(|_| bernulli(n, p, &mut rng))
                .map(|success| success as f32)
                .filter(|&success| mid - shift < success && success < mid + shift)
                .count();

            let from = (mid - shift).ceil() as u32;
            let to = (mid + shift).floor() as u32;

            let exact: f64 = (from..=to).map(|k| exact_benulli(n, p as f64, k)).sum();
            let approx: f64 = (from..=to).map(|k| approx_bernulli(n, p as f64, k)).sum();

            println!("n = {n}, from = {from}, to = {to}");
            println!(
                "with n = {n}, p = {p} expect bernulli with success from {} to {} events",
                mid - shift,
                mid + shift
            );
            println!("emulation result = {}", success as f32 / iters as f32);
            println!("exact result = {exact}");
            println!("approx result = {approx}");
        }
    }
}
