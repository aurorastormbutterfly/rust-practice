// Exercise 5: Basic String slice manipulation

pub fn exercise_five() {
    let greeting = "Hello, World!";
    
    // slicing: grab bytes from index 0 up to (but not including) index 5
    let hello_only = &greeting[..5];

    println!("Full greeting: {greeting}");
    println!("Extracted word: {hello_only}");
}
