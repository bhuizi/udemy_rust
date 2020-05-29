use std::ops::Mul;

struct Rectangle<T:Mul> {
    x:T,
    y:T
}

trait Shape<T> {
    fn area(&self)->T;
}

impl<T:Copy>Shape<T> for Rectangle<T> where T:Mul<Output=T> {
    fn area(&self)->T {
        self.x * self.y
    }

}
fn main() {

    let a = Rectangle{x:10, y:20};

    println!("Areaf of a Rectangle: {}", a.area());
}