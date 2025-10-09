#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(width: u32) -> Self {
        Self {
            width,
            height: width
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(60);

    println!(
        // 長方形の面積は、{}平方ピクセルです。
        "The area of the rectangle is {} square pixels.",
        square.area()
    );

    println!("{:?}", rect);
    println!("{:?}", square);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
