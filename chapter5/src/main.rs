fn main() {
    let mut a_string = String::from("First value");
    let user1 = User {
        name: a_string,
        age: 18,
        gender: false
    };
    // bellow will compile fail since a_string's ownership has been moved to user1, so that a_string is invalid now
    println!("{}", a_string);
}

struct User {
    name: String,
    age: u32,
    gender: bool
}
