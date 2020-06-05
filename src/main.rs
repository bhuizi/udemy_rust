use std::thread;
use std::sync::mpsc;

fn main() {

    let (tx,rx) = mpsc::channel();
    thread::spawn( move || {
        let value = String::from("Hello");
        tx.send(value).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("got the message: {}", received);
}