// hw13.rs

fn count_apples_and_oranges(
    s: i32,           // початок будинку
    t: i32,           // кінець будинку
    a: i32,           // координата яблуні
    b: i32,           // координата апельсинового дерева
    apples: &[i32],   // відстані падіння яблук
    oranges: &[i32],  // відстані падіння апельсинів
) {
    let apples_on_house = apples
        .iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    let oranges_on_house = oranges
        .iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count();

    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);
}

// Точка входу з прикладом
fn main() {
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
