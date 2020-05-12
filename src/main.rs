use std::io;

fn main() {

    println!("Enter a value");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error found");
    let input:i32 = input.trim().parse().expect("incorrect value entered by user");

    if input < 0 {
        println!("value is negative");
    }
    else if input > 0 {
        println!("value is positive");
    }
    else {
        println!("value is zero")
    }
}
