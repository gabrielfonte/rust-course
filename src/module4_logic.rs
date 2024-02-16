fn patterns_example_1() {
    println!("-- Patterns Example 1 --");
    let (x, y) = (2, 3);
    println!("This is X: {}", x);
    println!("This is Y: {}", y);
}

fn patterns_example_2() {
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
    let Person { name, age, .. } = p;

    println!("Name: {}", name);
    println!("Age: {}", age);
}

fn control_flow_example_1() {
    println!("-- Control Flow Example 1 - If --");
    let should_print = true;
    if should_print {
        println!("Printing!");
    }
}

fn control_flow_example_2() {
    println!("-- Control Flow Example 2 - If and Else --");
    let value = 10;
    if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }
}

fn control_flow_example_3() {
    println!("-- Control Flow Example 3 - If, Else if and Else --");
    let value = 10;
    if value == 0 {
        println!("Zero!");
    } else if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }
}

fn control_flow_example_4() {
    println!("-- Control Flow Example 4 - Loop --");
    let mut i = 10;
    loop {
        if i == 0 {
            break; // Use to break to get out from a Loop
        }
        println!("{i}...");
        i -= 1;
    }
    println!("Launch!");
}

fn control_flow_example_5() {
    println!("-- Control Flow Example 5 - While --");
    let mut i = 10;
    while i != 0 {
        println!("{i}...");
        i -= 1;
    }
    println!("Launch!");
}

fn control_flow_example_6() {
    println!("-- Control Flow Example 6 - For --");
    // Rev returns a reverse iterator
    for i in (1..=10).rev() {
        println!("{i}...");
    }
    println!("Launch!");
}

fn control_flow_example_7() {
    println!("-- Control Flow Example 7 - For with Continue --");
    // Prints only odd numbers
    for i in (1..=10).rev() {
        if i % 2 == 0 {
            //Skips to the next iteration
            continue;
        }
        println!("{i}...");
    }
    println!("Launch!");
}

fn pattern_matching_example_1() {
    println!("-- Pattern Matching Example 1 --");
    struct Plant {
        flowering: bool,
        mass: f64,
    }

    let plant = Plant {
        flowering: true,
        mass: 10.0,
    };

    let Plant { flowering, mass } = plant;

    println!("Flowering: {flowering}\nMass: {mass}");
}

// Creating an Enum
enum Meal{
    FishAndChips { chip_proportion: f64 },
    Hamburger { vegetarian: bool },
}

fn pattern_matching_example_2(){
    println!("-- Pattern Matching Example 2 --");

    // Creating a variable of
    let meal = Meal::Hamburger {
        vegetarian: true,
    };

    if let Meal::Hamburger { .. } = meal{
        println!("I had a hamburger!");
    }

    if let Meal::Hamburger { vegetarian: true } = meal {
        println!("I had a vegetarian hamburger!");
    }
}

fn pattern_matching_example_3(){
    println!("-- Pattern Matching Example 3 --");

    for n in 0..=5{
        match n {
            1 => println!("It was one!"),
            2 => println!("It was two!"),
            //or pattern
            3 | 4 => println!("It was a bit more than two"),
            high if high >= 5 => println!("It was a high number"),
            other => println!("It was {other}"),
        }
    }
}

fn pattern_matching_example_4(){
    println!("-- Pattern Matching Example 4 --");

    // Creating a variable of
    let meal = Meal::Hamburger {
        vegetarian: true,
    };

    // Match
    match meal {
        Meal::FishAndChips { chip_proportion} => {
            if chip_proportion > 0.5 {
                println!("I had some fish and plenty of chips!");
            } else if chip_proportion < 0.5 {
                println!("I had plenty of fish and some chips!")
            } else {
                println!("I had fish and chips!");
            }
        }
        Meal::Hamburger { vegetarian} => {
            if vegetarian {
                println!("I had a vegetarian hamburger!");
            } else {
                println!("I had a meaty hamburger!");
            }
        }
    }

    // Match reducing the nested ifs
    match meal {
        Meal::FishAndChips { chip_proportion} if chip_proportion > 0.5 => {
            println!("I had some fish and plenty of chips!");
        }
        Meal::FishAndChips { chip_proportion} if chip_proportion < 0.5 => {
            println!("I had plenty of fish and some chips!");
        }
        Meal::FishAndChips { chip_proportion} => {
            println!("I had fish and chips!");
        }
        Meal::Hamburger { vegetarian: true } => {
            println!("I had a vegetarian hamburger!");
        }
        Meal::Hamburger { vegetarian: false } => {
            println!("I had a meaty hamburger!");
        }
    }
}

fn pattern_matching_example_5(){
    println!("-- Pattern Matching Example 5 --");

    let mut meal = Meal::FishAndChips {
        chip_proportion: 0.6,
    };

    while let Meal::FishAndChips { chip_proportion} = meal {
        println!("Having fish and chips with chip proportion {:.2} ...", chip_proportion);
        if chip_proportion > 0.3 {
            // Order a meal with fewer chips
            meal = Meal::FishAndChips {
                chip_proportion: chip_proportion - 0.2,
            }
        } else {
            // Too fishy, let's get a hamburger next
            meal = Meal::Hamburger { vegetarian: true }
        }
    }
    println!("I'm so done with fish and chips!");
}

pub fn module_4() {
    println!("--------- Module 4 Lesson! --------");
    patterns_example_1();
    patterns_example_2();
    control_flow_example_1();
    control_flow_example_2();
    control_flow_example_3();
    control_flow_example_4();
    control_flow_example_5();
    control_flow_example_6();
    control_flow_example_7();
    pattern_matching_example_1();
    pattern_matching_example_2();
    pattern_matching_example_3();
    pattern_matching_example_4();
    pattern_matching_example_5();
    println!("-----------------------------------");
}
