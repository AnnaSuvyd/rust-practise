use std::io;

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn check_prime() {
    let mut input = String::new();
    println!("Введіть число для перевірки:");
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");
    let num: u64 = input.trim().parse().expect("Будь ласка, введіть ціле число");

    if is_prime(num) {
        println!("{} є простим числом.", num);
    } else {
        println!("{} не є простим числом.", num);
    }
}
