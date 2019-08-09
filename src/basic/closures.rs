fn main() {
    let doubler = |x| x * 2;
    let value = 5;
    let twice = doubler(value);
    print!("{} doubled is {}\n", value, twice);

    let big_closure = |b, c| {
        let z = b + c;
        z * twice
    };

    let some_number = big_closure(1, 2);
    print!("Result from closure: {}\n", some_number);
}