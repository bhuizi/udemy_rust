
struct Point<T, V> {
    x:T,
    y:V
}

fn main() {

    let a = Point{x:2, y:2};
    let b = Point{x:3.34, y:"hello"};
    let c = Point{x:true, y:false};

    println!("x:{}, y:{}", a.x, a.y);
    println!("x:{}, y:{}", b.x, b.y);
    println!("x:{}, y:{}", c.x, c.y);
}