use std::rc::Rc;

struct Person {
    name: Rc<String>
}

impl Person {
    fn name(x:Rc<String>) -> Person {
        Person{name:x}
    }
}

fn main() {

    let a = Rc::new("Hello".to_string());

    println!("Number of smart pointers: {}", Rc::strong_count(&a));

    let b = Person::name(a.clone());

    println!("{}", a);

    println!("Number of smart pointers: {}", Rc::strong_count(&a));
}