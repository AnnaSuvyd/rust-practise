pub fn draw_tree(levels: usize) {
    let mut height = 1;

    for i in 0..levels {
        for row in 0..(i + 2) {
            let spaces = levels + (i + 1) - row - 1;
            let stars = 2 * row + 1;

            let line: String = std::iter::repeat(' ')
                .take(spaces)
                .chain(std::iter::repeat('*').take(stars))
                .collect();

            println!("{}", line);
        }
    }
}
