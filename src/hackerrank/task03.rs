use std::io;

pub fn sum_large_array() {
    let mut input = String::new();

    // Зчитуємо кількість елементів
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let _n: usize = input.trim().parse().expect("Expected an integer");

    input.clear();

    // Зчитуємо сам масив
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Розділяємо рядок на числа і сумуємо, використовуючи i64 (64-бітні цілі числа)
    let sum: i64 = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("Expected a number"))
        .sum();

    println!("{}", sum);
}
