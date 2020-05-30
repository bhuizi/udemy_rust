fn call<F>(closure:F) where F:FnOnce(){
    closure();
}

fn main() {
    let mut x = 10;

    let value=|| {
        x=x+1;
        println!("value of x: {}", x);
    };
    call(value);
}