fn draw_tree(levels: usize) {
    for level in 1..=levels {
        for row in 0..level {
            let padding = " ".repeat(levels - row - 1);
            let stars = "*".repeat(1 + row * 2);
            println!("{}{}", padding, stars);
        }
    }
}


#[test]
fn main() {
    let triangles = 5;
    draw_tree(triangles);
}