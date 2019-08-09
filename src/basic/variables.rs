fn main() {
    let target = "world";
    let mut greeting = "hello";
    println!("{} {}", greeting, target);
    greeting = "How are you doing?";
    // target = "mate"; // will error
    println!("{}, {}", greeting, target);
}