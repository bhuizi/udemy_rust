fn reverse_order(tup:(i32, f64, bool, i32))-> (i32, bool, f64, i32) {
    let (a, b, c, d) = tup;
    (d, c, b, a)
}

fn main() {

    let tup:(i32, f64, bool, i32)=( 500, 2.4, true, 30);

    let new_tup = reverse_order(tup);

    println!("value is: {}", new_tup.1);

    // let (_a, _b, _c, _d) = tup;

    // println!("value is: {}", _b);

    // println!("value is: {}", tup.1);

    // let tup_2:(i32, f64, bool, i32, (i32, i32, i32)) = (500, 2.4, true, 30, (1, 2, 3));
    // println!("{}", (tup_2.4).2);
}
