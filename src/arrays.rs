// Exercise 4: Arrays and Basic Iteration
pub fn exercise_four() {
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    // .iter() lets us visit each item in a sequence (e.g. like a forEach)
    for day in days.iter() {
        println!("Weekday: {day}");
    }

    println!("Total weekdays: {}", days.len());
}
