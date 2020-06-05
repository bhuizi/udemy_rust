use std::rc::Rc;
use std::cell::RefCell;
fn main() {

    let value = Rc::new(RefCell::new(5));

    let b = Rc::new(Rc::clone(&value));

    *b.borrow_mut() += 10;

    println!("{:?}", b);
}