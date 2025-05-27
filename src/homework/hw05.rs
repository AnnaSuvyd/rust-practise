// Функція для обчислення найбільшого спільного дільника (GCD)
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Для прикладу — функція, яка зчитує два числа з клавіатури та виводить їх GCD
use std::io;

pub fn compute_gcd_from_input() {
    let mut input = String::new();

    println!("Введіть два числа, розділені пробілом:");
    io::stdin().read_line(&mut input).expect("Не вдалося зчитати рядок");

    let nums: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().expect("Очікувалося число"))
        .collect();

    if nums.len() != 2 {
        println!("Потрібно ввести саме два числа.");
        return;
    }

    let result = gcd(nums[0], nums[1]);
    println!("GCD({}, {}) = {}", nums[0], nums[1], result);
}
