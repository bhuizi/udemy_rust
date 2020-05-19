fn odd_num(i:i32)-> bool {

    return i % 2 == 1;
}


fn main() {

    let limit = 1000;
    let mut sum = 0;

    for i in 0.. {
        let square = i * i;
        if square > limit {
            break;
        }
        else if odd_num(square){
            sum = sum + square;
        }
    }
    println!("find sum off all squared odd numbers under 1000: {}", sum);

    let sum_odd_square=(0..)
        .map(|n|n*n)
        .take_while(|&square|square < limit)
        .filter(|&square|odd_num(square))
        .fold(0,|sum, square|sum + square);

    println!("find sum off all squared odd numbers under 1000: {}", sum_odd_square);
}
