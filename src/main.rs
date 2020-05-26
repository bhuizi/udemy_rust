struct Rectangle {
    width: i32,
    height: i32
}

struct Triangle {
    side_1: i32,
    side_2: i32,
    side_3: i32
}

impl Rectangle {
    fn area(&self){
        println!("Area of rectangle: {}", self.width * self.height);
    }
    fn sum_rectangle(&self){
        println!("Sum of side of rectangle: {}", 2 * self.width + 2 * self.height );
    }
}

impl Triangle {
    fn sum_triangle(&self) {
        println!("Sum of sides of triangle:{}", self.side_1 + self.side_2 + self.side_3);
    }
}

fn main() {

    let rec = Rectangle{width: 50, height: 70};

    rec.area();

    rec.sum_rectangle();

    let tri = Triangle{side_1: 10, side_2: 30, side_3: 40};

    tri.sum_triangle();
}
