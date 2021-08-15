use std::io;

fn main() {

    println!("Please input a fahrenheit temperature");
    let fahrenheit = get_std_input();

    let fahrenheit :f32 = fahrenheit.trim().parse().expect("You must input a right temperature number");
    println!("fahrenheit: {} ºF", fahrenheit);
    println!("Celsius of your input is {} ªC", fahrenheit_to_celsius(fahrenheit));

    println!("Please input a day of fibonacci");
    let fibonacci_day = get_std_input();
    let fibonacci_day :i32 = fibonacci_day.trim().parse().expect("You must input a right day number");
    println!("Fibonacci of day {} is {}", fibonacci_day, fibonacci(fibonacci_day));

    lyric_to_the_12_days_of_christmas();
}

fn get_std_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Err when read_line");
    input
}

fn fahrenheit_to_celsius(f :f32) -> f32 {
    (f - 32.0) * 0.5556
}

fn fibonacci(day :i32) -> i32 {
    if (day == 1 || day == 2) {
        1
    } else {
        sum(fibonacci(day - 2), fibonacci(day -1))
    }
}

fn sum(a :i32, b :i32) -> i32 {
    a + b
}

fn lyric_to_the_12_days_of_christmas() {
    let a = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let b = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine drummers drumming",
        "ten pipers piping",
        "eleven ladies dancing",
        "twelve lords a-leaping"
    ];
    for index in (0..12) {
        println!("On the {} day of Christmas, my true love sent to me", a[index]);
        for cur in (0..index + 1).rev() {
            print!("{}", b[cur]);
            if (cur == 1) {
                print!(", and ");
            } else if (cur > 1) {
                print!(", ");
            }
        }
        println!(".");
    }
}


// On the first day of Christmas, my true love sent to me
// A partridge in a pear tree
// On the second day of Christmas, my true love sent to me
// Two turtle doves
// And a partridge in a pear tree
// On the third day of Christmas, my true love sent to me
// Three French hens, two turtle doves
// And a partridge in a pear tree
// On the fourth day of Christmas, my true love sent to me
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree
// On the fifth day of Christmas, my true love sent to me
// Five gold rings, four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree
// On the sixth day of Christmas, my true love sent to me
// Six geese a-laying, five gold rings, four calling birds
// Three French hens, two turtle doves
// And a partridge in a pear tree
// On the seventh day of Christmas, my true love sent to me
// Seven swans a-swimming, six geese a-laying, five gold rings
// Four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree
// On the eighth day of Christmas, my true love sent to me
// Eight maids a-milking, seven swans a-swimming, six geese a-laying
// Five gold rings, four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree
// On the ninth day of Christmas, my true love sent to me
// Nine drummers drumming
// On the tenth day of Christmas, my true love sent to me
// Ten pipers piping
// Nine drummers drumming, ten pipers piping
// Drumming, piping, drumming, piping
// Eight maids a-milking, seven swans a-swimming, six geese a-laying
// Five gold rings, four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree
// On the eleventh day of Christmas, my true love sent to me
// Eleven ladies dancing, ten pipers piping, nine drummers drumming
// Eight maids a-milking, seven swans a-swimming, six geese a-laying
// Five gold rings, four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree
// On the twelfth day of Christmas, my true love sent to me
// Twelve lords a-leaping, eleven ladies dancing, ten pipers piping
// Nine drummers drumming, eight maids a-milking
// Seven swans a-swimming, six geese a-laying
// And five gold rings, four calling birds, three French hens, two turtle doves
// And a partridge in a pear tree, and a partridge in a pear tree

