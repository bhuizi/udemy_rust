
trait Foo {
    fn method(&self)->String;
}

impl Foo for i32 {
 fn method(&self)->String {
     format!("i32: {}", *self)
 }
}

impl Foo for String {
    fn method(&self)->String {
        format!("String: {}", *self)
    }
}

fn bar(z:&dyn Foo) {
    println!("{}", z.method()); 
}

// fn bar<T:Foo>(z:T) {
//     println!("{}", z.method());
// }

fn main() {
    let a = 30;
    let b = "Hello".to_string();

    bar(&a);
    bar(&b);
}