fn number() -> i32 {
    5
}

fn multy() -> i32 {
    5 * 5
}

fn multyply(number: i32, number2: i32) -> i32 {
    let _result = number * number2;
    return _result;
}

fn main() {
    println!("Hello, number {} !", number());
    println!("Hello, number {} !", multy());
    println!("Hello, number {:?} !", multyply(6, 7));
}
