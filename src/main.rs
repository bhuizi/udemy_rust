#[derive(Debug,Clone)]
// #[derive(Debug,Clone, Copy)]

struct A {
    x: i32
}

struct B {
    y: i32
}

fn main() {

    let _a = A{x: 10};
    let _b = B{y: 20};

    let _c = _a.clone();
    // with Copy
    // let _c = _a;

    println!("{:?}", _a);
}