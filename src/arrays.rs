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
