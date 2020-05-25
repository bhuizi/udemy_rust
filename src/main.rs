struct Rectangle {
    width: i32,
    height: i32
}

//tuple
struct Triangle(i32, i32, i32);

fn area(rectangle:&Rectangle)->i32 {
    let area = rectangle.width * rectangle.height;
    return area;
}

fn main() {

    let value = Rectangle{width:60, height:70};
    let sides = Triangle(10, 20, 30);

    println!("Rectangle width: {}", value.width);
    println!("Rectangle height: {}", value.height);

    println!("Triangle 0 index: {}", sides.0);
    println!("Triangle 0 index: {}", sides.1);

    let rec_1 = area(&value);

    println!("Area of a rectangle is: {}", rec_1);
}
