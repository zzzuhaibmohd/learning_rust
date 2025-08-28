fn main() {
    // ✅ Works fine
    let good: i32 = "100".parse().expect("Failed to parse number!");
    println!("Parsed with expect(): {good}");

    // ❌ Panics, but with OUR custom message
    let bad: i32 = "oops".parse().expect("Custom error: Not a valid number!");
    println!("This line never runs: {bad}");
}