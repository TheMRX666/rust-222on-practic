#[test]
fn main(){
    const WIDTH: usize = 15;
    const HEIGHT: usize = 15;
    let mut output = String::new();

    let max_i = HEIGHT - 1;
    let max_j = WIDTH - 1;
    let center_i = HEIGHT / 2;
    let center_j = WIDTH / 2;

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let star = (i == 0 || i == max_i || j == 0 || j == max_j)
                || (j == i || j == max_j - i)
                || (HEIGHT % 2 != 0 && i == center_i && (j == center_j || j == center_j - 1));
            output.push(if star { '*' } else { ' ' });
        }
        output.push('\n');
    }

    print!("{}", output);
}