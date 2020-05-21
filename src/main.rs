fn average_age(array:&[i32]) {

    let mut sum = 0;

    for i in 0..10 {
        sum = sum + array[i];
    }

    println!("average age of a person is: {}", sum/10);

}

fn sum(array_one:&[i32], array_two:&[i32])->[i32; 5] {

    let mut tmp:[i32; 5] = [0, 0, 0, 0, 0];

    for i in 0..5 {
        tmp[i] = array_one[i] + array_two[i];
    }
    return tmp;
}

fn main() {

    // let age:[i32; 10] = [ 25, 30, 50, 45, 70, 43, 22, 33, 90, 40];
    let array_one:[i32; 5] = [ 5, 10, 20, 30 , 50];
    let array_two:[i32; 5] = [10, 20, 30, 35, 40];

    let array_three:[i32; 5] = sum(&array_one, &array_two);

    for i in 0..5 {
        println!("value is: {}", array_three[i]);
    }
    // average_age(&age);
}
