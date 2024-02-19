use rand::random;

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
enum Meal {
    FishAndChips { chip_proportion: f64 },
    Hamburger { vegetarian: bool },
}

fn pattern_matching_example_2() {
    println!("-- Pattern Matching Example 2 --");

    // Creating a variable of
    let meal = Meal::Hamburger { vegetarian: true };

    if let Meal::Hamburger { .. } = meal {
        println!("I had a hamburger!");
    }

    if let Meal::Hamburger { vegetarian: true } = meal {
        println!("I had a vegetarian hamburger!");
    }
}

fn pattern_matching_example_3() {
    println!("-- Pattern Matching Example 3 --");

    for n in 0..=5 {
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

fn pattern_matching_example_4() {
    println!("-- Pattern Matching Example 4 --");

    // Creating a variable of
    let meal = Meal::Hamburger { vegetarian: true };

    // Match
    match meal {
        Meal::FishAndChips { chip_proportion } => {
            if chip_proportion > 0.5 {
                println!("I had some fish and plenty of chips!");
            } else if chip_proportion < 0.5 {
                println!("I had plenty of fish and some chips!")
            } else {
                println!("I had fish and chips!");
            }
        }
        Meal::Hamburger { vegetarian } => {
            if vegetarian {
                println!("I had a vegetarian hamburger!");
            } else {
                println!("I had a meaty hamburger!");
            }
        }
    }

    // Match reducing the nested ifs
    match meal {
        Meal::FishAndChips { chip_proportion } if chip_proportion > 0.5 => {
            println!("I had some fish and plenty of chips!");
        }
        Meal::FishAndChips { chip_proportion } if chip_proportion < 0.5 => {
            println!("I had plenty of fish and some chips!");
        }
        Meal::FishAndChips { chip_proportion } => {
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

fn pattern_matching_example_5() {
    println!("-- Pattern Matching Example 5 --");

    let mut meal = Meal::FishAndChips {
        chip_proportion: 0.6,
    };

    while let Meal::FishAndChips { chip_proportion } = meal {
        println!(
            "Having fish and chips with chip proportion {:.2} ...",
            chip_proportion
        );
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

fn pattern_matching_example_6() {
    println!("-- Pattern Matching Example 6 --");

    // For example 1
    let tuples: [(usize, &'static str); 3] = [(1, "red"), (2, "white"), (3, "blue")];

    for (numbering, color) in tuples {
        println!("Color #{numbering} is {color}!");
    }

    // For example 2
    let colors = ["red", "white", "blue"];

    for (index, color) in colors.into_iter().enumerate() {
        let numbering = index + 1;
        println!("Color #{numbering} is {color}!");
    }
}

fn statements_and_expressions_example_1() {
    println!("-- Statements and Expressions Example 1 --");

    // Generate random brownies number
    let brownies_eaten = random::<u8>() % 10;

    // if statement
    if brownies_eaten == 0 {
        println!("I had no brownies today.");
    } else {
        println!("I had at least one brownie today.");
    }

    // if statement, reducing duplication
    let quantifier;
    if brownies_eaten == 0 {
        quantifier = "no brownies";
    } else {
        quantifier = "at least one brownie";
    }
    println!("I had {quantifier} today");

    // using if as an expression
    let quantifier = if brownies_eaten == 0 {
        "no brownies"
    } else {
        "at least one brownie"
    };
    println!("I had {quantifier} today.");
}

fn statements_and_expressions_example_2() {
    println!("-- Statements and Expressions Example 2 --");

    // Generate random brownies
    let brownies_eaten = random::<u8>() % 3;

    // Using match as an expression
    let quantifier = match brownies_eaten {
        0 => "no brownies",
        1 => "a brownie",
        _ => "multiple brownies",
    };
    println!("I had {quantifier} today");
}

fn statements_and_expressions_example_3() {
    println!("-- Statements and Expressions Example 3 --");

    let (a, b) = (2, 1);

    let x = if a > b {
        println!("It was true!");
        1
    } else {
        2
    };

    println!("The X result is {x}!");
}

fn statements_and_expressions_example_4() {
    println!("-- Statements and Expressions Example 4 --");

    // Scope evaluates to nothing
    let x;
    {
        println!("`x` has no value yet!");
        x = 3;
    }

    // Scope evaluates to result of the expression
    let y = {
        println!("`y` has no value yet!");
        3
    };
}

fn statements_and_expressions_example_5() {
    println!("-- Statements and Expressions Example 5 --");

    // Loop as expression
    let mut i = 0;
    let x = loop {
        if i > 7 {
            break i;
        }
        i += i * 2 + 1;
    };
    println!("{x}");
}

fn functions_and_closures_example_1(){
    println!("-- Functions and Closures Example 1 --");

    // Declaring a function
    fn u32_add(a: u32, b: u32) -> u32 {
        return a + b;
    }
    println!("1 + 2 = {}", u32_add(1, 2));

    // You could also do:
    fn u32_add_alternative(a: u32, b: u32) -> u32 {
       a + b
    }
    println!("2 + 2 = {}", u32_add(2, 2));

    // It is allowed to define functions inside other functions:
    fn f(n: u32) -> u32 {
        fn g(n: u32) -> u32 {
            n + 1
        }

        g( n * 2)
    }
    println!("{}", f(3));
}

fn functions_and_closures_example_2(){
    println!("-- Functions and Closures Example 2 --");

    struct X(&'static str);
    // An implementation block for the type `X`.
    impl X {
        // An associated function
        fn associated_fn() -> &'static str {
            "I am always the same!"
        }
        // A method
        fn method(self: &Self) -> &'static str {
            self.0
        }
    }
    // Call a function associated to the type `X`.
    println!("{}", X::associated_fn());
    // Create an instance of X and call a method on the instance.
    let instance = X("My value depends on an instance of `X`!");
    println!("{}", instance.method());
    // Create another instance of X
    let instance2 = X("See now my value is different!");
    println!("{}", instance2.method());
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
    pattern_matching_example_6();
    statements_and_expressions_example_1();
    statements_and_expressions_example_2();
    statements_and_expressions_example_3();
    statements_and_expressions_example_4();
    statements_and_expressions_example_5();
    functions_and_closures_example_1();
    functions_and_closures_example_2();
    println!("-----------------------------------");
}
