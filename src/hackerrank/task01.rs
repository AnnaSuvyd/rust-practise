use std::io;

fn main() {
    let mut num_str_1 = String::new();
    let mut num_str_2 = String::new();
    io::stdin().read_line(&mut num_str_1).expect("Помилка зчитуання");
    io::stdin().read_line(&mut num_str_2).expect("Помилка зчитування");
    let num_1: i32 = num_str_1.trim().parse().expect("Помилка перетворення ");
    let num_2: i32 = num_str_2.trim().parse().expect("Ошибка перетворення");
    println!("{}", num_1 + num_2);
}
