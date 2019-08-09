/*
    Rust Code Create by : Jirawat Harnsiriwatanakit (jirawat.h@kazekim.com)
*/

fn main() {
    let status = req_status();
    match status {
        200 => print!("Success\n"),
        404 => print!("Not found\n"),
        other => {
            print!("Request failed with code: {}\n", other);
            // get response from cache
        }
    }
}

fn req_status() -> u32 {
    500
}