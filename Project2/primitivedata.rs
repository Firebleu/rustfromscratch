// Primitive Data Types
// 4 Types : int, float, bool, char
//
//
//Integer (int)
//  Signed integers (for +/- numbers): i8/i16/i32/i64/i128
//  Unsigned integers (for - numbers): u8/u16/u32/u64/u128
fn main() {
    let x: i32 = -85; //can be also 97
    let y: u64 = 154; //cannot be -154
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Diff between i32 and i64
    // i32 = 32 bit and i64 = 64 bit
    // About the range :
    //  min number for i32 : -2147483647 and max -2147483647
    //  min number for i64 : - 9223372036854775807 and max 9223372036854775807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Max value for i32: {}", e);
    println!("Max value for i64; {}", i);

    //Floats (Floating Point Types) = Number with fractional part
    //f32, f64
    let t: f32 = 4.789;
    println!("Value of f32: {}", t);

    //Boolean Values = True or False
    let is_sunny: bool = true;
    println!("It is sunny ? {}", is_sunny);
