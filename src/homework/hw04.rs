// Константи: висота ромба (має бути непарна, в межах 10–80)
const H: usize = 21;
const W: usize = H; // Для ромба ширина = висоті

pub fn draw_diamond() {
    let mut output = String::new();

    let mid = H / 2;

    for y in 0..H {
        for x in 0..W {
            let dist = if y <= mid { y } else { H - y - 1 };
            if x == mid - dist || x == mid + dist {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
