fn call<F>(closure:F) where F:Fn(){
    closure();
}

fn main() {
    let x = 10;

    let value=|| {
        println!("value of x: {}", x);
    };
    call(value);
}