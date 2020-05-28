struct Droppable {
    name: String
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("dropping: {}", self.name);
    }
}

fn main() {
    let a = Droppable{name: "Hello".to_string()};

    {
        let b = Droppable{name: "Rust".to_string()};
        {
            let c = Droppable{name: "C++".to_string()};
        }
    }



}