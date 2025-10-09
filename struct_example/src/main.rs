struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        // 長方形の面積は、{}平方ピクセルです。
        "The area of the rectangle is {} square pixels.",
        area(rect)
    );
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
