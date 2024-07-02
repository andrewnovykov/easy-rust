//TYPES

fn main() {
    let number = 5;

    println!("The value of number is: {}", number);
    println!("The value of number is: {}", number as u8);
    // println!("The value of number is: {}", number as char); // ERROR
    println!("The value of number is: {}", number as u8 as char);

    let number_2: u8 = 100;
    println!("The value of number is: {}", number_2 as char);

    let slice = "Hello!";
    println!(
        "The slice is: {} bytes and also {} chars ",
        slice.len(),
        slice.chars().count()
    );

    let slice_2 = "привіт!";
    println!(
        "The slice is: {} bytes and also {} chars ",
        slice_2.len(),
        slice_2.chars().count()
    );
}
