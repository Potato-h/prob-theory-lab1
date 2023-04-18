use rand::prelude::*;

fn dist(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}

fn main() {
    let mut rng = rand::thread_rng();

    let n = 100000;
    let success = (0..n)
        .map(|_| {
            (
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                rng.gen::<f32>(),
            )
        })
        .map(|(x1, y1, x2, y2)| (((x1 + x2) / 2.0, (y1 + y2) / 2.0), dist(x1, y1, x2, y2)))
        .filter(|((x, y), r)| x + r < 1.0 && x - r > 0.0 && y + r < 1.0 && y - r > 0.0)
        .count();

    println!("Probality of event = {}", success as f32 / n as f32);
}
