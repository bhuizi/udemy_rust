fn main() {

    // let a = vec![1, 2, 3, 4];
    // println!("value is: {}", a[2]);

    // for i in a {
    //     println!("value is: {}", i)
    // }

    let mut b = vec![1, 2, 3, 4];
    b.push(10);

    for i in &b {
        println!("value is: {}", i);
    }

    println!("length of vector is: {}", b.len());
}
