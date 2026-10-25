fn main() { 
    let name = "Fern";
    println!("Hello, {name}!");

    // exercise 1: mutable lets
    let mut score = 10;
    println!("{name}'s initial score: {score}");

    score += 5;
    println!("{name}'s updated score: {score}");

    // exercise 2: tuples
    // a tuple of (&str, i32, f64)
    let item = ("Notebook", 3, 4.50);
    println!("Item: {}", item.0);
    println!("Quantity: {}", item.1);
    println!("Price per unit: {}", item.2);

    // Rust does not implicitly convert numbers. Quantity needs to be cast to f64 to multiply with ppu. 
    let total = (item.1 as f64) * item.2;
    // the {xyz:.2} is used to tell println how many decimal points to show.
    println!("Total cost: £{total:.2}");

    // exercise 3: Simple for loop and Range
    for number in 1..=5 {
        if number % 2 == 0 {
            println!("{number} is even");
        } else {
            println!("{number} is odd");
        }
    }
} // end of main

