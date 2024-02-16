use std::mem::size_of;

fn data_types(){
    //Scalar Types
    println!("Scalar Types: ");

    //Variable declaration
    let signed_8: i8 = -5;
    let unsigned_8: u8 = 3;
    let signed_16: i16 = -500;
    let unsigned_16: u16 = 1000;
    let signed_32: i32 = -1000000;
    let unsigned_32: u32 = 10000000;
    let signed_64: i64 = -10000000;
    let unsigned_64: u64 = 50000000;
    let signed_128: i128 = -100000000;
    let unsigned_128: u128 = 500000000;
    let isize_var: isize = 4;
    let usize_var: usize = 4;

    //Print
    println!("Integers:");
    println!("Signed i8 {} and Unsigned u8 {}", signed_8, unsigned_8);
    println!("Signed i16 {} and Unsigned u16 {}", signed_16, unsigned_16);
    println!("Signed i32 {} and Unsigned u32 {}", signed_32, unsigned_32);
    println!("Signed i64 {} and Unsigned u64 {}", signed_64, unsigned_64);
    println!("Signed i128 {} and Unsigned u128 {}", signed_128, unsigned_128);
    println!("Isize has {} bytes and Usize has {} bytes and both are the size of the pointer", isize_var, usize_var);
}

pub fn module_3(){
    println!("--------- Module 2 Lesson! --------");
    data_types();
    println!("-----------------------------------");
}