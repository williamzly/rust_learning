fn main() {
    println!("Hello, world!");
    another_method();
    let plus = plus(1, 2);
    println!("plus is {}", plus);
}

fn another_method() {
    println!("another_method");
}

fn plus(a :i32, b :i32) -> i32 {
    a + b
}

