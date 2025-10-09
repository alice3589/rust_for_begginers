fn main() {
    let width = 30;
    let height = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです。
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
