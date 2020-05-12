use std::io;

fn main() {

    println!("Enter a gender:");
    let mut gender_input = String::new();
    io::stdin().read_line(&mut gender_input).expect("error found");
    let value = gender_input.trim();

    if value == "Male" {
        println!("Enter age: ");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("error found");
        let age:i32 = age.trim().parse().expect("no value found");

        if age < 19 {

            println!("Qualified Men U-19 Team");
        }
        else {
            println!("Not qualified");
        }
    }
    else {
        println!("Not Qualified");
    }
}
