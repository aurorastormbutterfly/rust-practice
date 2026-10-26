# Rust Learning Roadmap & Course Log

A structured, practical log of exercises and milestones for learning Rust from first principles.

---

## Learning Strategy & Themes

To make concepts memorable and practical without heavy mathematics or dry business logic, each topic is anchored around two light-hearted, recurring narrative themes:

- **Demonstrations (The Chaotic Mountain Expedition):** Concepts are introduced through the mishaps and practicalities of a mountain hike - soggy socks, pocket essentials, midges, and trail snacks.
- **Challenges (Sanctuary Shenanigans):** Exercises are completed through the lens of a quirky wildlife rescue centre - mischievous crows hoarding shiny objects, badgers on strict nap rotas, and escaped ferrets.

Each step follows a steady cadence:

1. Direct, low-level breakdown of the Rust mechanic.
2. Demonstration code (Expedition Tracker).
3. Targeted challenge (Sanctuary Shenanigans).

---

## Phase 0: Foundations & Warm-Ups (Completed)

- **Initial Setup (`src/main.rs`):**
  - Cargo project generation, compilation via `cargo run`, and formatted printing with `println!`.
- **Warm-Ups Module (`src/warm_ups.rs`):**
  - **Exercise 1 (Mutability):** Default immutability vs. explicit `mut` flags; basic integer operations. Passing parameters (`&str`) across module boundaries.
  - **Exercise 2 (Basic Types & Precision):** Compound data with tuples (`(&str, i32, f64)`); explicit numeric casting (`as f64`); formatting decimals with `{variable:.2}`.
  - **Exercise 3 (Control Flow & Ranges):** Inclusive ranges (`1..=5`), modulus arithmetic (`% 2`), and condition branching with `if` / `else`.
- **Arrays Module (`src/arrays.rs`):**
  - **Exercise 4 (Basic Arrays & Iteration):** Fixed collections, counting elements with `.len()` (returning `usize`), and iterating via `.iter()`.
- **Strings Module (`src/strings.rs`):**
  - **Exercise 5 (String Slicing):** String slices (`&str`), byte-range indexing (`&greeting[..5]`), and references.

---

## Phase 1: Collections, Memory Shapes & Basic Structs

- **Step 1: Fixed Arrays vs. Dynamic Vectors (`Vec<T>`)**
  - Fixed stack arrays (`[T; N]`) vs. growable heap buffers (`Vec<T>`).
  - Creating vectors with `vec![]` and modifying with `.push()`.
- **Step 2: Structs & Custom Data Types**
  - Defining named structs to replace loose tuples.
  - Field types, instantiation, and field access syntax.
- **Step 3: Associated Functions & Methods (`impl` blocks)**
  - Attaching logic to types; constructors (`Self::new`) vs. instance methods (`&self`, `&mut self`).
  - Deriving standard traits (`#[derive(Debug)]`).
- **Step 4: Vector Operations & Safe Access**
  - Popping, removing, and slicing collections.
  - Safe index lookups with `.get()` to avoid runtime panics.

---

## Phase 2: User Input, Ownership & The Borrow Checker

- **Step 5: Interactive Terminal Input**
  - Standard I/O via `std::io::stdin()`.
  - Buffer handling, trimming whitespace (`.trim()`), and parsing text to numbers (`.parse()`).
- **Step 6: Ownership & Moves**
  - Stack vs. heap allocations; copy types vs. move types.
  - Scope, variable lifetime, and automatic resource cleanup (RAII).
- **Step 7: Borrowing & References**
  - Shared references (`&T`) vs. exclusive mutable references (`&mut T`).
  - The aliasing rule: multiple readers OR one writer, never both.

---

## Phase 3: Robust Control Flow & Error Handling

- **Step 8: Enums & Pattern Matching**
  - Defining custom enums with rich data payloads.
  - Exhaustive checks with `match` blocks and shorthand branching with `if let`.
- **Step 9: Working with Nullability (`Option<T>`)**
  - Eliminating null bugs with `Some(T)` and `None`.
  - Safe unpacking: `unwrap_or`, `match`, and default fallbacks.
- **Step 10: Handling Recoverable Errors (`Result<T, E>`)**
  - `Ok(T)` vs. `Err(E)`.
  - Error bubbling using the `?` operator instead of exceptions.

---

## Phase 4: Data Transformation & Functional Iteration

- **Step 11: Iterators in Depth**
  - Iterating by reference (`.iter()`), by mutable reference (`.iter_mut()`), and by value (`.into_iter()`).
  - Functional pipelines: `.map()`, `.filter()`, and `.take()`.
- **Step 12: Collecting & Aggregating**
  - Gathering transformed items with `.collect()`.
  - Reductions: `.sum()`, `.count()`, and `.fold()`.
- **Step 13: Strings Revisited**
  - Distinguishing `String` (heap-allocated, resizable) from `&str` (borrowed slice).
  - Safe character boundaries and formatted strings via `format!`.

---

## Phase 5: Architecture & Code Organisation

- **Step 14: Multi-File Modules & Encapsulation**
  - Deep module trees, folder structures, and visibility controls (`pub`, `pub(crate)`).
- **Step 15: Traits (Shared Behaviour)**
  - Defining interfaces via traits.
  - Implementing standard library traits like `std::fmt::Display`.
- **Step 16: Basic Generics**
  - Writing flexible functions and structs that accept any type satisfying a trait.

---

## Phase 6: Practical Projects & Tooling

- **Step 17: File System Operations**
  - Reading text files with `std::fs::read_to_string` and writing outputs safely.
- **Step 18: External Crates & Ecosystem**
  - Adding dependencies in `Cargo.toml`.
  - Working with JSON data using `serde` and `serde_json`.
  - Writing automated unit tests using `#[test]`.
