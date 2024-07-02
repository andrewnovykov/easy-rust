//more types

fn main() {
    // let number = 355655436345; //the literal `355655436345` does not fit into the type `i32`
    let number = 355655436345_i64;
    println!("number is {}", number);

    let number_fl = 3.5;
    println!("number is {}", number_fl);

    let _my_float: f32 = 3.5;
    let _my_float_2: f64 = 3.5;
    // let sum = my_float + my_float_2; //cannot add `f64` to `f32`

    let _my_float_3: f32 = 3.5;
    let _my_float_4 = 3.5;
    let _sum = _my_float_3 + _my_float_4;
    println!("sum is {}", _sum);
}
