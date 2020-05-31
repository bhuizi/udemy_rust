fn main() {
    let message=String::from("welcome to Rust language");

    println!("After replace:{}", message.replace("Rust", "React"));

    let split_message=String::from("welcome+to+rust+language");

    let token:Vec<&str>=split_message.split("+").collect();

    println!("{}", token[2]);

    let trim_message=String::from("     welcome to Rust language");

    println!("After trim: {}", trim_message.trim());

    let chars_message=String::from("welcome to Rust language");

    match chars_message.chars().nth(2) {
        Some(value) => println!("character found a second index: {}", value),
        // _=> println!("character not found")
        None=> println!("character not found")
    }

}