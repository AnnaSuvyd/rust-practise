use std::io;

pub fn min_max_sum() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка зчитування");

    let numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Введіть ціле число"))
        .collect();

    if numbers.len() != 5 {
        panic!("Потрібно ввести саме 5 чисел");
    }

    let total_sum: u64 = numbers.iter().sum();

    let mut min_sum = u64::MAX;
    let mut max_sum = 0u64;

    for i in 0..5 {
        let sum_four = total_sum - numbers[i];
        if sum_four < min_sum {
            min_sum = sum_four;
        }
        if sum_four > max_sum {
            max_sum = sum_four;
        }
    }

    println!("{} {}", min_sum, max_sum);
}
