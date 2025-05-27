use std::io;

pub fn diagonal_difference(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut primary_sum = 0;
    let mut secondary_sum = 0;

    for i in 0..n {
        primary_sum += matrix[i][i];
        secondary_sum += matrix[i][n - i - 1];
    }

    (primary_sum - secondary_sum).abs()
}

pub fn run_diagonal_difference() {
    let mut input = String::new();

    // Зчитування розміру матриці
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Expected integer");

    let mut matrix = Vec::new();

    // Зчитування кожного рядка матриці
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Expected number"))
            .collect();

        if row.len() != n {
            panic!("Matrix must be square");
        }

        matrix.push(row);
    }

    let result = diagonal_difference(matrix);
    println!("{}", result);
}
