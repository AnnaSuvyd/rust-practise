// Константи для ширини (W) та висоти (H) конверта
const W: usize = 40;
const H: usize = 20;

pub fn draw_envelope() {
    let mut output = String::new();

    for y in 0..H {
        for x in 0..W {
            let ch = if y == 0 || y == H - 1 {
                // Верхній та нижній край
                '*'
            } else if x == 0 || x == W - 1 {
                // Лівий та правий край
                '*'
            } else if x * H == y * W || (W - x - 1) * H == y * W {
                // Діагоналі від кутів
                '*'
            } else {
                // Все інше - пробіл
                ' '
            };
            output.push(ch);
        }
        output.push('\n');
    }

    print!("{}", output);
}
