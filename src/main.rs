use std::collections::HashMap;

fn main() {
    let mut student=HashMap::new();

    student.insert("A210", "John");
    student.insert("A211", "Bob");
    student.insert("A212", "Rock");
    student.insert("A213", "Kim");
    student.insert("A214", "Jenny");

    println!("Number of students: {}", student.len());

    match student.get("A210") {
        Some(value)=>println!("value against key is: {}", value),
        _=> println!("value not found")
    }

    student.insert("A214", "Alex");

    for (key, value) in &student {
        println!("{}:{}", key, value);
    }

    println!("value contained in hashmap: {}", student.contains_key("A214"));
}