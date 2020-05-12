
fn main() {

    let side_one = 10;
    let side_two = 10;
    let side_three = 10;

    if side_one == side_two && side_two == side_three {
        println!("It's an equilateral triangle");
    }
    else if side_one == side_two || side_two == side_three {
        println!("It's an isosceles triangle");
    }
    else {
        println!("It's a Scalene triangle");
    }
}
