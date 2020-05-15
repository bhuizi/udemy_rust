
fn main() {

    let mut n = 1;

    while n <= 10 {
        if n % 2 == 0 {
            println!("even number is: {}", n);
        }
        else {
            println!("odd number is: {}", n);
        }
        n = n + 1;
    }
}
