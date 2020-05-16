
fn main() {

    let mut number = 5;

    match number {
        1 => println!("number is one"),
        2 => println!("number is two"),
        3 | 5 => println!("number is: {}", number),
        10..=20 => println!("number is: {}", number),
        _ => println!("nothing has matched")
    }
}
