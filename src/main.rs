trait Shape {
    fn area(&self)->f64;
}

trait Addition<T> {
    fn add(&self)->T;
}

struct Rectangle {
    x: f64,
    y: f64
}

struct Circle {
    radius: f64
}

impl Shape for Rectangle {
    fn area(&self)->f64 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self)->f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl Addition<i32> for Vec<i32> {
    fn add(&self)->i32 {
        let mut result = 0;

        for i in self {
            result = result + i;
        }
        return result;
    }
}

fn main () {

    let rec = Rectangle{x: 10.24, y: 15.54};
    let circ = Circle{radius: 2.24};
    let arr = vec![1, 2, 3, 4, 5];

    println!("area of rectangle is: {}", rec.area());
    println!("area of circle is: {}", circ.area());
    println!("{}", arr.add());
}