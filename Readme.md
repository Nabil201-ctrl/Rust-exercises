. Simple (Core Syntax)

Print a welcome message with your name using variables.

Swap two numbers without a temporary variable.

Take a number and print if it's even or odd.

Convert Celsius to Fahrenheit.

Reverse a string without using .rev() directly (manual loop).

Generate 10 Fibonacci numbers using a loop.

Make a simple calculator using match.

Count vowels and consonants in a sentence.

Find the largest number in an array manually.

Simulate a coin flip (random true/false using rand crate).

2. Beginner (Ownership & Functions)

Create a function that takes ownership of a string and prints it.

Fix a Rust program that fails due to borrowing rules.

Write a function that borrows a vector and returns its sum.

Build a CLI that asks for your age and prints a message without cloning values unnecessarily.

Make a function that returns multiple values using tuples.

Build a password strength checker (length + contains number + contains symbol).

Create a struct Person { name, age } and implement fn new().

Implement impl method to increase age on a mutable struct.

Build a program that demonstrates move vs borrow with comments.

Create an enum TrafficLight and print actions based on state.

3. Intermediate (Collections & Error Handling)

Read user input safely and handle invalid numbers using Result.

Create a HashMap to count occurrences of words in a paragraph.

Implement a stack (push, pop, peek) using Vec.

Implement a queue (enqueue, dequeue) manually.

Load a text file and print the longest line.

Create a function that may fail and propagate errors using ?.

Build a mini todo list saved in memory using a vector of structs.

Add delete and edit functionality to the todo list.

Validate an email format using regex crate.

Build a number guessing game with proper error handling.

4. Advanced (Traits, Lifetimes, Generics)

Create a trait Speak and implement it for multiple structs.

Write a generic function that finds the largest item in a list.

Create a struct that holds references and fix lifetime errors.

Implement Iterator trait for a custom struct.

Build a plugin-like system using traits (e.g., payment methods).

Implement a logger using trait objects (dyn Trait).

Build a caching system using generics and HashMap.

Create a function that returns references and annotate lifetimes correctly.

Create a custom smart pointer and implement Deref.

Compare static dispatch (generics) vs dynamic dispatch (dyn) performance using loops.

5. Complex / Real-World / Expert

Build a multithreaded file downloader simulator (no real internet, spawn threads that write chunks).

Build a thread-safe shared counter using Arc<Mutex<>>.

Create a small web API using Axum or Actix.

Add authentication middleware to your Rust API.

Build a mini in-memory database with insert, query, delete, and indexing.

Implement serialization/deserialization with serde and JSON storage.

Build a real CLI tool with subcommands using clap.

Build a plugin executor that loads dynamic libraries (.so or .dll) — optional but advanced.

Rebuild one of your past ideas (like trivia, booking, or scan app) but in Rust backend.

Build a secure hash storage system that salts and hashes input (use argon2 crate safely).

Implement rate limiting manually on a Rust server.

Build a basic OS process monitor CLI (CPU/memory usage using sysinfo crate).

Implement an async task scheduler (like cron jobs) using tokio.

Build a websocket chat server.

Make it scale using channels (mpsc) and async workers.