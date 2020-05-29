
struct Point<T> {
    x:T,
    y:T
}

fn main() {

    let a = Point{x:2, y:2};
    let b = Point{x:3.34, y:4.32};
    let c = Point{x:true, y:false};

    println!("x:{}, y:{}", a.x, a.y);
    println!("x:{}, y:{}", b.x, b.y);
    println!("x:{}, y:{}", c.x, c.y);
}