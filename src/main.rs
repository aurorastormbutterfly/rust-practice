// Tell Rust to load ./warm-ups.rs (& other files) as a module
mod warm_ups;
mod arrays;
mod strings;

fn main() { 
    let name = "Fern";
    println!("Hello, {name}!");

    println!("\n---- Running exercises from warm_ups module ---");
    warm_ups::exercise_one(name);
    warm_ups::exercise_two();
    warm_ups::exercise_three();

    println!("\n---- Running exercises from arrays module ---");
    arrays::exercise_four();
    arrays::demo_pockets_and_rocks();
    arrays::exercise_six();
    arrays::exercise_six_refactored();

    println!("\n---- Running exercises from string module ---");
    strings::exercise_five();

} // end of main

