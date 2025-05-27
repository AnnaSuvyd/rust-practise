use std::io::{self, BufRead};

fn is_palindrome_number(num: i64) -> bool {
    let s = num.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Зчитуємо число з консолі
    let input = lines.next().unwrap().unwrap();
    let num: i64 = input.trim().parse().expect("Введіть коректне ціле число");

    if is_palindrome_number(num) {
        println!("true");
    } else {
        println!("false");
    }
}
