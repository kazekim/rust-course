/*
    Rust Code Create by : Jirawat Harnsiriwatanakit (jirawat.h@kazekim.com)
*/

use std::collections::HashMap;

fn main() {
    let mut fruits = HashMap::new();
    fruits.insert("Apple", 3);
    fruits.insert("Mango", 6);
    fruits.insert("Orange", 2);
    fruits.insert("Avocado", 7);
    for (k, v) in &fruits {
        println!("I Got {} {}", v, k);
    }

    fruits.remove("Orange");
    let old_avocado = fruits["Avocado"];
    fruits.insert("avocado", old_avocado + 5);
    println!("\nI now have {} avocados", fruits["Avocado"]);
}