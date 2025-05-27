use std::io;

pub fn plus_minus() {
    let mut input = String::new();
    
    // Зчитуємо розмір масиву
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Expected a number");

    input.clear();
    // Зчитуємо сам масив
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Expected number"))
        .collect();

    if arr.len() != n {
        panic!("Невідповідність розміру масиву");
    }

    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;

    for &num in &arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    let total = n as f64;
    println!("{:.6}", positive_count as f64 / total);
    println!("{:.6}", negative_count as f64 / total);
    println!("{:.6}", zero_count as f64 / total);
}
