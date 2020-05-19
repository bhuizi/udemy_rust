
// fn sum (a:i32, b:i32) {
//     let total = a + b;
//     println!("sum is: {}", total);
// }

fn max (a:i32, b:i32, c:i32)->i32 {
    if a > b && a > c {
        return a;
    }
    else if b > a && b > c {
        return b;
    }
    else {
        return c;
    }
}

fn main() {

    // sum(10, 10);
    let value = max(50, 20, 30);
    println!("max value is: {}", value);
}
