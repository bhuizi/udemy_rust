use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut file=File::create("output.txt")
        .expect("Error in creating a file");

    file.write_all(b"Welcome to Rust programming language")
        .expect("Error in writing a file");
}