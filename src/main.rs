
mod programming {

    pub fn rust(){
        println!("Welcome to Rust Language");
    }

    pub fn c_plus_plus() {
        println!("Welcome to C++ Language");
    }

    pub mod data_base {
        pub fn sql() {
            println!("Welcome to SQL Language");
        }
    }
}

fn main() {
    programming::rust();
    programming::c_plus_plus();
    programming::data_base::sql();
}
