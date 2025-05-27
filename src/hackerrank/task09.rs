use std::io::{self, BufRead};

fn count_tallest_candles(candles: &[i32]) -> i32 {
    let max_height = candles.iter().max().unwrap_or(&0);
    candles.iter().filter(|&&h| h == *max_height).count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Читаємо розмір масиву (не використовується безпосередньо)
    let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Читаємо висоти свічок у вектор
    let candles: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = count_tallest_candles(&candles);
    println!("{}", result);
}
