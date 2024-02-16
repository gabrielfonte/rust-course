fn patterns_example_1(){
    println!("-- Patterns Example 1 --");
    let (x, y) = (2, 3);
    println!("This is X: {}", x);
    println!("This is Y: {}", y);
}

fn patterns_example_2(){
    println!("-- Patterns Example 2 --");

    // Define the `Person` type.
    struct Person {
        name: &'static str,
        age: u32,
        likes_brownies: bool,
    }
    // Create a `Person` from its parts.
    let p = Person {
        name: "Mick",
        age: 30,
        likes_brownies: true,
    };
    // Deconstruct the `Person` back into its parts,
    // omitting fields other than `name` and `age`.
    let Person {
        name, age, ..
    } = p;

    println!("Name: {}", name);
    println!("Age: {}", age);
}

pub fn module_4(){
    println!("--------- Module 4 Lesson! --------");
    patterns_example_1();
    patterns_example_2();
    println!("-----------------------------------");
}