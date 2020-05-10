use std::io;

fn main() {

    // String value
    // println!("enter a value");

    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("error found");

    // println!("value entered by user is {}", input);

    println!("enter a value");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error found");

    let input:i32 = input.trim().parse().expect("incorrect value entered by user");

    println!("value entered by user is {}", input);
}
