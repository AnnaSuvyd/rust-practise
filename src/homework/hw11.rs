use rand::Rng;

/// Генерує випадковий вектор довжиною `n` зі значеннями в діапазоні [10..=99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

/// Знаходить мінімальну суму пари сусідніх елементів у векторі
fn min_adjacent_sum(data: &[i32]) -> Option<(i32, (i32, i32))> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_pair = (data[0], data[1]);

    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
        }
    }

    Some((min_sum, min_pair))
}

fn main() {
    let vec = gen_random_vector(20);
    println!("Згенерований вектор: {:?}", vec);

    match min_adjacent_sum(&vec) {
        Some((sum, (a, b))) => {
            println!("Мінімальна сума пари: {} + {} = {}", a, b, sum);
        }
        None => {
            println!("Недостатньо елементів для обчислення.");
        }
    }
}
