# Arrays & Dynamic Vectors

## Core Concepts

In Rust, sequential collections fall into two fundamental categories based on how and where they exist in memory:

### 1. Fixed Arrays (`[T; N]`)

- Fixed size defined at compile time (`N` elements of type `T`).
- Allocated on the stack: extremely fast access with zero dynamic heap allocation overhead.
- Immutably sized: elements cannot be added or removed at runtime.
- Standard indexing uses zero-based `usize` indices (e.g. `array[0]`).

### 2. Dynamic Vectors (`Vec<T>`)

- Resizable array stored on the heap.
- Can grow or shrink at runtime using methods such as `.push()` and `.pop()`.
- Must be declared as mutable (`let mut`) to append or remove items.
- Can be initialised empty via `Vec::new()` or pre-populated using the `vec![]` macro.

---

## Demonstration: The Chaotic Mountain Expedition

This example illustrates fixed stack allocation for non-negotiable pocket items versus a dynamic vector for rocks gathered along the path.

```rust
pub fn demo_pockets_and_rocks() {
    // 1. Fixed Array: The 3 items that must stay in your jacket pockets
    // Type signature: [&str; 3]
    let pocket_essentials = ["Compass", "Whistle", "Emergency Flapjack"];

    println!("--- Pocket Check ---");
    println!("First essential: {}", pocket_essentials[0]);
    println!("Total essentials: {}", pocket_essentials.len());

    // 2. Dynamic Vector: A backpack pocket for trail finds
    // Marked mut so items can be appended at runtime
    let mut rock_collection = vec!["Shiny Quartz", "Suspiciously Round Pebble"];

    // Appending more finds along the trail
    rock_collection.push("Fossilised Moss (maybe?)");
    rock_collection.push("Slightly Green Slate");

    println!("\n--- Rock Hoard Report ---");
    for rock in rock_collection.iter() {
        println!("Stashed: {rock}");
    }
    println!("Total weight hazard: {} rocks", rock_collection.len());
}
```

## Challenge: Sanctuary Shenanigans (Exercise 6)

### Scenario

Barnaby the badger maintains an immovable, three-part daily routine that never varies. Meanwhile, Corvid the crow spends his afternoons gathering shiny trinkets to hide away in his nest hoard.

### Requirements

Inside `src/arrays.rs`, implement `pub fn exercise_six()`:

1. **Fixed Array (Barnaby's Routine):**
   - Create a fixed array with exactly 3 string slice entries:
     - Slot 0: `"Morning Grump"`
     - Slot 1: `"Afternoon Snooze"`
     - Slot 2: `"Midnight Snack Raid"`
   - Print Barnaby's first habit using direct index lookup (`[0]`).
   - Print the total number of habits using `.len()`.

2. **Dynamic Vector (Corvid's Hoard):**
   - Create a mutable vector `Vec<&str>` containing the starting items:
     - `"Shiny milk bottle top"`
     - `"Lost brass screw"`
   - Use `.push()` to add two additional items:
     - `"Someone's shiny car key"`
     - `"Stolen tin foil wrapper"`

3. **Output & Iteration:**
   - Iterate over the vector with a `for` loop (using `.iter()`) to print each hoarded item.
   - Print the final total count of Corvid's treasures.
