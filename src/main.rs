fn main() {
    let x = 10;

    let value=|mut x| {
        x=x+1;
        println!("value of x: {}", x);
    };

    value(x);

    println!("value o x:{}", x);

}