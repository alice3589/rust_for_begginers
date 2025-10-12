#[derive(Debug)]

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

fn main() {
    println!("Hello, world!");
    let green = Color::Green;
    let yellow = Color::Yellow;
    println!("Green Color Code: {}", color_to_str(green));
    println!("Yellow Color Code {}", color_to_str(yellow));
    find_maybe_number(Some(5));
    find_maybe_number(None);

}

// enum Option {
//     Some(u32),
//     None,
// }

fn find_maybe_number(maybe_number: Option<u32>) {
    match maybe_number {
        Some(number) => println!("found {}", number),
        None => println!("Nothing found."),
    }
}

fn color_to_str(color: Color) -> String {
    // RED #FF0000
    // BLUE #0000FF
    // GREEN #00FF00
    match color {
        Color::Red => "#FF0000".to_string(),
        Color::Blue => "#0000FF".to_string(),
        Color::Green => "#00FF00".to_string(),
        Color::Yellow => "#FFFF00".to_string(),
    }
}
