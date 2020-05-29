use std::fmt;

fn something<T:fmt::Debug>(z:T){

    println!("{:?}", z);
}

fn main() {

    let a = 10.24;

    something(a);
}