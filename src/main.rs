
fn main() {

    // for i in 0..5 {
    //     println!("value of is is: {}", i);
    // }

    // Print 5 rows and 10 stars

    // for row in 0..5 {
    //     for stars in 0..10 {
    //         print!("*");
    //     }
    //     println!();
    // }

    for row in 0..5 {
        for stars in 0..row + 1 {
            print!("*");
        }
        println!();
    }
}
