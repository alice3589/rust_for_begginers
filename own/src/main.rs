fn main() {
    {
        let s = "hello";
        println!("{}", s);
    }

    // println!("{}", s);

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);
}
