fn divide(numerator: f64, denominator: f64)->Option<f64> {
    if denominator == 0.0 {
        None
    }
    else {
        Some(numerator/denominator)
    }
}

fn main() {

    // let result = divide(3.0, 2.0);
    let result = divide(3.0, 1.0);

    // println!("Result is {:?}", result);

    match result {
        Some(x) => println!("value is: {}", x),
        None => println!("Cannot divide by 0"),
    }
}
