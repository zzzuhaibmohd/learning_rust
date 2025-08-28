fn main() {
    // ✅ Works fine
    let good: i32 = "42".parse().unwrap();
    println!("Parsed with unwrap(): {good}");

    // ❌ This will panic at runtime
    let bad: i32 = "hello".parse().unwrap();
    println!("This line never runs: {bad}");
}
