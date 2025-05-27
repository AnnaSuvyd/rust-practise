fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 == v2 {
        if x1 == x2 {
            "YES"
        } else {
            "NO"
        }
    } else {
        let numerator = x2 - x1;
        let denominator = v1 - v2;

        if denominator != 0 && numerator % denominator == 0 && (numerator / denominator) >= 0 {
            "YES"
        } else {
            "NO"
        }
    }
}

fn main() {
    // Зразок 0
    let (x1, v1, x2, v2) = (0, 3, 4, 2);
    println!("{}", kangaroo(x1, v1, x2, v2)); // Очікувано: YES

    // Зразок 1
    let (x1, v1, x2, v2) = (0, 2, 5, 3);
    println!("{}", kangaroo(x1, v1, x2, v2)); // Очікувано: NO
}
