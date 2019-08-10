/*
    Rust Code Create by : Jirawat Harnsiriwatanakit (jirawat.h@kazekim.com)
*/

fn main() {
    let numbers : [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let floats = [0.1f64, 0.2f64, 0.3f64];

    //Wrong
    //let floats = [0.1f64, 0.2, 0.3];

    println!("Number: {}", numbers[5]);
    println!("Float: {}", floats[2]);
}