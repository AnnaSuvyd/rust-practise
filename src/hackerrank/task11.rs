/// Функція, яка приймає список оцінок і повертає новий список з округленими оцінками
fn round_grades(grades: Vec<i32>) -> Vec<i32> {
    grades
        .into_iter()
        .map(|grade| {
            if grade < 38 {
                grade
            } else {
                let next_multiple_of_5 = ((grade / 5) + 1) * 5;
                if next_multiple_of_5 - grade < 3 {
                    next_multiple_of_5
                } else {
                    grade
                }
            }
        })
        .collect()
}

fn main() {
    let grades = vec![73, 67, 38, 33];
    let rounded = round_grades(grades);
    for grade in rounded {
        println!("{}", grade);
    }
}
