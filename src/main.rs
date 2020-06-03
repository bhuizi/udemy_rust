fn print_sum(v:&Vec<i32>) {
    println!("{:?}", v[2] + v[3]);
}

fn main() {
    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    print_sum(&v);

    println!("{}", v[3]);
}