fn main() {

    let arr:[[i32; 4 ];3] = [[1, 2, 3, 4], [4, 5, 6, 7], [8, 9, 10, 11]];

    for rows in 0..3 {
        for cols in 0..4 {
            println!("value is :{}", arr[rows][cols]);
        }
    }
    // println!("value is: {}", arr[1][3]);
}
