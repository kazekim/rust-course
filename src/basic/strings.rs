/*
    Rust Code Create by : Jirawat Harnsiriwatanakit (jirawat.h@kazekim.com)
*/

use std::string::ToString;

fn main() {
    let question = "How are you?";
    let person: String = "Kim".to_string();
    let namaste = String::from("नमस्ते");

    print!("{} {} {}\n", namaste, question, person);
}