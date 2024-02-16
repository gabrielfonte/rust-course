fn overview(){
    println!("The topics this course introduces are the following: ");
    println!("- INTRODUCTION about RUST Programming Language");
    println!("- TYPES in RUST and how to create your own custom type");
    println!("- LOGIC: How to implement Logic and Organize the Code");
    println!("- DATA OWNERSHIP: This new thing that RUST introduces called data ownership and how to share data throughout the code");
    println!("- ERROR HANDLING: How to handle errors in RUST using Monadic Types and Exceptions");
    println!("- TRAITS AND GENERICS: Trait Details and Trait Objects");
    println!("- STANDARD LIBRARY: Dynamically Sized Arrays, String Types, Box, Associative Types (incl. HashMap and BTreeMap), Smart Pointers, Arguments and the Environment, The Filesystem, Traits");
    println!("- LIBRARIES AND APPLICATIONS: How to organize the code in a high level and how to reuse pieces of code");
    println!("- TESTING!");
    println!("- COMMONLY USED EXTERNAL LIBRARIES: Error Handling, Logging, Testing, Serialization and Deserialization, Command Line Applications");
    println!("- COMMONLY USED EXTERNAL TOOLS: Includes cargo-edit, cargo-audit, cargo-udeps");
    println!("- GOING DEEPER: Web3/Solana, Embedded Devices");
}

fn greetings(){
    let course_url: &str = "https://careerbooster.io/courses/full-solana-and-rust-programming-course-for-beginners";
    println!("Greetings! This is the Rust Course from Teachable, available at {}.\nI made this notebook to help myself learning this language, but feel free to use for your own and if you like the course, take it in the Career Boost Website.", course_url);
}

pub fn module_1(){
    greetings();
    println!("--------- Module 1 Lesson! --------");
    overview();
    println!("-----------------------------------");
}