use chrono::{Local, Timelike};

// Exercise 4: Arrays and Basic Iteration
pub fn exercise_four() {
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    // .iter() lets us visit each item in a sequence (e.g. like a forEach)
    for day in days.iter() {
        println!("Weekday: {day}");
    }

    println!("Total weekdays: {}", days.len());
}

// Demo 1: pockets and rocks
pub fn demo_pockets_and_rocks() {
    println!("\n---- Demo: Pockets and Rocks ----");
    // 1. Fixed Array: the three items that must stay in your jacket pockets
    // Type signature: [&str; 3]

    let pocket_essentials = ["Compass", "Whistle", "Emergency Flapjack"];

    println!("\n--- Pocket Check ---");
    println!("First essential: {}", pocket_essentials[0]);
    println!("Total essentials: {}", pocket_essentials.len());

    // 2. Dynamic Vector: A backpack pocket for trail finds
    // Marked mut so items can be appended at runtime
    let mut rock_collection = vec!["Shiny Quartz", "Suspiciously Round Pebble"];

    // Appending more finds along the trail
    rock_collection.push("Fossilised Moss (Maybe?)");
    rock_collection.push("Slightly Green Slate");

    println!("\n--- Rock Hoard Report ---");
    for rock in rock_collection.iter() {
        println!("Stashed: {rock}");
    }
    println!("Total weight hazard: {} rocks", rock_collection.len());
}

// Challenge 1: Sancturary Shenanigans (Exercise 6)
pub fn exercise_six() {
    println!("\n---- Challenge 1: Animal Observations ----");
    let barnabys_routine = ["morning grump", "afternoon snooze", "midnight snack raid"];

    println!("\n --- Badger Enclosure Observations---");
    println!("Barnaby engages in: {}", barnabys_routine[0]);
    println!("Barnaby has completed 1 of {} actions in his routine.", barnabys_routine.len());

    println!("\n --- Corvid Activity Report --- ");
    let mut corvids_collection = vec!["shiny milk bottle top", "lost brass screw"];
    println!("Corvid picks up and stashes someone's shiny car keys");
    corvids_collection.push("someone's stolen car keys");
    println!("Corvid picks up and stashes a stolen tin foil hat");
    corvids_collection.push("stolen tin foil hat");

    for item in corvids_collection.iter() {
        println!("Corvid admires the {item}");
    };
}   

// Challenge 1: Refactored and updated

fn select_action() -> (usize, u32) {
    let current_hour = Local::now().hour();
    if current_hour == 0 {
        (2, 3)
    } else if current_hour < 12 {
        (0, 1)
    } else {
        (1, 2)
    }

}

fn define_article(action: &str) -> &str {
    if action.starts_with('a') {
        "an"
    } else {
        "a"
    }
}

fn stash_trinket<'a>(trinket: &'a str, collection: &mut Vec<&'a str>){
    collection.push(trinket);
    println!("Corvid picks up and stashes {}", trinket);
}

fn admire(name: &str, collection: Vec<&str>){
    for item in collection.iter() {
    println!("{name} admires the {item}");
    };
}

pub fn exercise_six_refactored() {
    // create Barnaby's routine
    let barnabys_routine = ["morning grump", "afternoon snooze", "midnight snack raid"];

    // call helper function for observations and make them depend on time of day
    let (action_index, count) = select_action();
    let action = barnabys_routine[action_index];

    println!("\n --- Badger Enclosure Observations---");
    println!("Barnaby engages in {} {}", define_article(action) ,action);
    println!("Barnaby has completed {} of {} actions in his routine.", count, barnabys_routine.len());

    println!("\n --- Corvid Activity Report --- ");
    // create Corvid's collection
    let mut corvids_collection = vec!["shiny milk bottle top", "lost brass screw"];
    // call help function to add items
    stash_trinket("someone's stolen car keys", &mut corvids_collection);
    stash_trinket("stolen tin foil hat", &mut corvids_collection);

    //call helper function to observe behaviour
    admire("Corvid", corvids_collection);
}