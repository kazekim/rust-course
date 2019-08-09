/*
    Rust Code Create by : Jirawat Harnsiriwatanakit (jirawat.h@kazekim.com)
*/

struct Color(u8, u8, u8);

fn main() {
    let white = Color(255, 255, 255);

    //You can pull them out by index
    let red = white.0;
    let green = white.1;
    let blue = white.2;

    print!("Red value: {}\n", red);
    print!("Green value: {}\n", green);
    print!("Blue value: {}\n", blue);

    let orange = Color(255, 165, 0);

    // You can also destructure the fields directly
    let Color(r, g, b) = orange;
    print!("R: {}, G: {}, B: {} (orange)\n", r, g, b);

    let g = 123;
    // Can also ignore fields while destructuring
    let Color(r, _, b) = orange;

    print!("R: {}, G: {}, B: {} (orange)\n", r, g, b);
}