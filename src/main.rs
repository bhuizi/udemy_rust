

fn func(i:i32) ->i32 {

    i + 1
}

fn main() {

    let i = 10;

    let closure=|| {
        i+1
    };

    println!("value of i is: {}", closure());
    println!("value of i is: {}, func", func(i));
}