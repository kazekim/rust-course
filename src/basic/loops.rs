/*
    Rust Code Create by : Jirawat Harnsiriwatanakit (jirawat.h@kazekim.com)
*/

fn main() {
    let mut x = 1024;
    loop {
        if x < 0 {
            break;
        }
        print!("{} more runs to go\n", x);
        x -= 1;

    }
}