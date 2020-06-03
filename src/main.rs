fn max<'a>(x:&'a i32, y:&'a i32) -> &'a i32 {
    if *x > *y {
        x
    }
    else {
        y
    }
}

fn main() {

    {
        let a = 10;
            {
                let b = 20;
                let _c = max(&a, &b);
                println!("{}", _c);
            }
    }
}