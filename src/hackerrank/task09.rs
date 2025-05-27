use std::io::{self, BufRead};

fn rotate_string(s: &str, k: usize) -> String {
    let len = s.len();
    if len == 0 {
        return String::new();
    }
    let k = k % len;
    format!("{}{}", &s[len - k..], &s[..len - k])
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Зчитуємо рядок
    let s = lines.next().unwrap().unwrap();

    // Зчитуємо число зсуву k
    let k: usize = lines.next().unwrap().unwrap().trim().parse().expect("Введіть ціле число");

    let result = rotate_string(&s, k);

    println!("{}", result);
}
