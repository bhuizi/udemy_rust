use std::rc::Rc;

fn func(x:Rc<String>){
    println!("{}", x);
}

fn main() {

    let a = Rc::new("Hello".to_string());

    func(a.clone());

    println!("{}", a);
}