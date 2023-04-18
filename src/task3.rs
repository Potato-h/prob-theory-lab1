use prob_theory_lab1::{c, factorial};
use rand::prelude::*;

fn get_num(lambda: f64, rng: &mut ThreadRng) -> i32 {
    let p: f64 = rng.gen();
    let mut x = 0.0;

    for i in 0.. {
        x += f64::exp(-lambda) * lambda.powi(i) / factorial(i as u32);

        if p < x {
            return i;
        }
    }

    0
}

fn main() {
    let n = 100000;
    let mut rng = rand::thread_rng();

    for i in [3, 5, 10] {
        for k in [2, 4, 7] {
            for lambda in [2.0, 5.0] {
                for mu in [3.0, 4.0] {
                    let j = i + k;
                    let (success, all) = (0..n)
                        .map(|_| (get_num(lambda, &mut rng), get_num(mu, &mut rng)))
                        .filter(|&(x, y)| x + y == j)
                        .fold((0, 0), |acc, (x, _)| (acc.0 + ((x == i) as u32), acc.1 + 1));

                    let formula = c(j as u32, i as u32) * mu.powi(j - i) * lambda.powi(i)
                        / (mu + lambda).powi(j);

                    println!(
                        "Emulate experiment with: i = {i}, j = {j}, lambda = {lambda}, mu = {mu}"
                    );

                    println!("Result of expriment = {}", success as f32 / all as f32);
                    println!("Result of formula = {formula}");
                }
            }
        }
    }
}
