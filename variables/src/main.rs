fn main() {
    let mut x: i32 = 5;  
    const CONSTANT: usize = 100;
    println!("The value of CONSTANT is: {}", CONSTANT);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5; // 5
    
    let y = y + 1; // 6

    {
        let y = y * 2; // 12
        let z = 5; // このブロックの中だけで使用可能な変数
        println!("The value of y in the inner scope is: {}", y);
        println!("The value of z is {}", z);
    }

    // println!("The value of z is: {}", z); これだとエラーになる

    println!("The value of y is: {}", y);
}
