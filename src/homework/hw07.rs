use std::io;

pub fn toggle_case() {
    let mut input = String::new();
    println!("Введіть рядок:");
    io::stdin().read_line(&mut input).expect("Помилка зчитування");

    let toggled: String = input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect();

    println!("{}", toggled.trim_end()); // видаляємо \n в кінці
}
