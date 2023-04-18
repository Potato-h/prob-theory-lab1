pub fn factorial(n: u32) -> f64 {
    (1..=n).fold(1.0, |x, y| x * y as f64)
}

pub fn c(n: u32, k: u32) -> f64 {
    factorial(n) / factorial(n - k) / factorial(k)
}
