fn diverge()->! {
    panic!("Problem Occurred");
}

fn main() {

    diverge();

    println!("This line wont be reached");

}
