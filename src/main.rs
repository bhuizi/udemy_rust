fn main() {

    let mut d = String::from("hello world");

    println!("length of this string is: {}", d.len());

    // single char
    // d.push('w');

    d.push_str(", welcome to the rust programming language");

    println!("message is: {}", d);

    for string in d.split_whitespace(){
        print!("{}", string);
    }

    print!("does d contain Hello: {} ", d.contains("Hello"));
}