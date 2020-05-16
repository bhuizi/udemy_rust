fn main() {

    let mut result:Option<i32> = Some(0);

    while let Some(i) = result {
        if i > 9 {
            println!("value is greater then 9");
            result = None;
        }
        else {
            println!("value is less then 9");
            result = Some(i + 1);
        }

    }
    // loop {
    //     match result {
    //         Some(i) => {
    //             if i > 9 {
    //                 println!("value is greater then 9");
    //                 result = None;
    //             }
    //             else {
    //                 println!("value is less then 9");
    //                 result = Some(i + 1);
    //             }
    //         },
    //         None => {
    //             break;
    //         }
    //     }
    // }
}
