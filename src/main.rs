use std::thread;
use std::time::Duration;

fn main () {

    let handle = thread::spawn(||
     {
         for i in 0..10 {
             println!("new thread:{}", i);
             thread::sleep(Duration::from_secs(2));
         }
     });

     for i in 0..5 {
         println!("main thread:{}", i);
         thread::sleep(Duration::from_secs(2));
     }

     handle.join();
}