use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut file=File::open("output.txt")
        .expect("cannot open file");

    let mut content=String::new();

    file.read_to_string(&mut content)
        .expect("error in reading a file");

    println!("content of file is: {}", content);
}