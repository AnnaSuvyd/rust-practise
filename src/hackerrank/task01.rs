use std::io;

fn main() {
    // Объявление переменных для ввода
    let mut num_str_1 = String::new();
    let mut num_str_2 = String::new();

    // Читаем входные данные
    io::stdin().read_line(&mut num_str_1).expect("Ошибка чтения");
    io::stdin().read_line(&mut num_str_2).expect("Ошибка чтения");

    // Преобразуем строки в числа
    let num_1: i32 = num_str_1.trim().parse().expect("Ошибка преобразования");
    let num_2: i32 = num_str_2.trim().parse().expect("Ошибка преобразования");

    // Вывод суммы чисел
    println!("{}", num_1 + num_2);
}
