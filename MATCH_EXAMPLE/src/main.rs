#[derive(Debug)]

enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    println!("Hello, world!");
    let green = Color::Green;
    println!("Green Color Code: {}", color_to_str(green));
}

fn color_to_str(color: Color) -> String {
    // RED #FF0000
    // BLUE #0000FF
    // GREEN #00FF00
    match color {
        Color::Red => "#FF0000".to_string(),
        Color::Blue => "#0000FF".to_string(),
        Color::Green => "#00FF00".to_string(),
    }
}
