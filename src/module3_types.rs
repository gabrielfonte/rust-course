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

    //Float Notations: 2022f64, 6., 17.0, 2.002E1
    let float_32: f32 = 500f32;
    let float_64: f64 = 2.002E1;

    println!("Floats:");
    println!("Float 32: {}", float_32);
    println!("Float 64: {}", float_64);

    let boolean: bool = true;
    println!("Booleans: {}", boolean);

    let char: char = '\u{1F680}';
    println!("Char: chars accepts any UTF-8 including emojis like this {}", char);

    let string: &str = "This is a String! \u{26A1}";
    println!("String: {}", string);

    let tuple: (bool, u32, f64) = (true, 2, 3.0);
    println!("Tuple: [ {} {} {} ]", tuple.0, tuple.1, tuple.2);

    struct MyTuple(bool, u32, f64);
    let tuple = MyTuple(false, 2, 3.0);
    println!("Tuple struct: [ {} {} {} ]", tuple.0, tuple.1, tuple.2);

    type MyTupleAlias = (bool, u32, f64);
    let tuple2: MyTupleAlias = (false, 2, 3.0);
    println!("Tuple alias: [ {} {} {} ]", tuple2.0, tuple2.1, tuple2.2);

    println!("Structs:");
    struct MyStruct {
        should_do_groceries: bool,
        birth_year: u32,
        height_in_meters: f64,
    };

    let my_struct = MyStruct {
        should_do_groceries: true,
        birth_year: 1992,
        height_in_meters: 1.79,
    };
    println!("MyStruct");
    println!("should_do_groceries: {}", my_struct.should_do_groceries);
    println!("birth_year: {}", my_struct.birth_year);
    println!("height_in_meters: {}", my_struct.height_in_meters);

    println!("Enum:");
    #[derive(Debug)]
    enum CardinalDirection {
        North,
        East,
        South,
        West,
    }
    let d = CardinalDirection::East;
    println!("Cardinal Direction Enum: {:#?}", d);

    println!("Tagged Union:");
    enum Shape {
        Square {
            side: f64
        },
        Rectangle {
            width: f64,
            height: f64,
        },
        Circle {
            radius: f64,
        },
    }
    let s = Shape::Rectangle {
        width: 800.0,
        height: 60.0
    };
    match s {
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        },
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        },
        Shape::Circle { radius } => {
            println!("A circle of radius {} and diameter {}!", radius, radius * 2.0);
        }
    }
}

pub fn module_3(){
    println!("--------- Module 2 Lesson! --------");
    data_types();
    println!("-----------------------------------");
}