fn main() {

    // let result:Option<i32>=Some(10);
    let result:Option<i32>=None;

    if let Some(i) = result {
        println!("matched {:?}", i);
    }
    else {
        println!("not matched");
    }
}
