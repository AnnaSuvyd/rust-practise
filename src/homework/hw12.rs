// hw12.rs

// 1. Функція count_permutation: мінімальна кількість переносу вантажу
fn count_permutation(shipments: &Vec<u32>) -> Result<usize, &'static str> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    // 3. Перевірка: чи можливо забезпечити однакову кількість вантажу
    if total % n != 0 {
        return Err("Неможливо рівномірно розподілити вантаж між кораблями");
    }

    let target = total / n;
    let mut moves = 0;

    // Підрахунок переносу лише там, де вантажу більше за середній
    for &load in shipments {
        if load > target {
            moves += (load - target) as usize;
        }
    }

    Ok(moves)
}

// 5. Функція генерації коректного Vec<u32>, розподіленого рівномірно
fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let base: u32 = rng.gen_range(10..100);
    (0..n).map(|_| base).collect()
}

// 2. main-функція для демонстрації
fn main() {
    // Приклад вхідних даних
    let shipments = vec![10, 20, 30, 40];

    match count_permutation(&shipments) {
        Ok(moves) => println!("Потрібно перенести {} одиниць вантажу", moves),
        Err(msg) => println!("Помилка: {}", msg),
    }

    // Приклад генерації коректного вектора
    let generated = gen_shipments(5);
    println!("Згенеровані дані: {:?}", generated);

    // Перевірка вирівнювання для згенерованих даних
    match count_permutation(&generated) {
        Ok(moves) => println!("Згенерований вектор вже збалансований, переноси: {}", moves),
        Err(msg) => println!("Помилка: {}", msg),
    }
}
