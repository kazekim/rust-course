/*
    Rust Code Create by : Jirawat Harnsiriwatanakit (jirawat.h@kazekim.com)
*/

fn main() {
    // does not include 10
    print!("Normal ranges: ");
    for i in 0..10 {
        print!("{}, ", i);
    }

    println!();
    print!("Inclusive ranges: ");
    // count until 10
    for i in 0..=10 {
        print!("{}, ", i);
    }
}