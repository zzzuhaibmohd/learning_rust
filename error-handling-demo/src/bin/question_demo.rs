// To use `?`, our main must return Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ✅ Works fine
    let good: i32 = "7".parse()?;   // if error happens, it bubbles up
    println!("Parsed with ?: {good}");

    // ❌ This will return Err instead of panic
    let bad: i32 = "NaN".parse()?;  
    println!("This line never runs: {bad}");

    Ok(())
}
