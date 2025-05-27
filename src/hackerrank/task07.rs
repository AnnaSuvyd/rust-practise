use std::io;

pub fn print_stairs(n: usize) {
    for i in 1..=n {
        // Спочатку друкуємо (n - i) пробілів
        let spaces = n - i;
        let line: String = std::iter::repeat(' ')
            .take(spaces)
            .chain(std::iter::repeat('#').take(i))
            .collect();

        println!("{}", line);
    }
}
