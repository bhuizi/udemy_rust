use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {

    let (tx,rx) = mpsc::channel();
    thread::spawn( move || {
        let value = vec![
            String::from("welcome"),
            String::from("to"),
            String::from("Rust"),
            String::from("!!!"),
        ];

        for i in value {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("{:?}", received);
    }

}