use std::fmt;

// ---------- Data Model ----------
struct Note {
    title: String, // lives on the heap; the pointer/len/cap are on stack
    body: String,  // lives on the heap
}

// Pretty-print a Note
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.title, self.body)
    }
}

// ---------- Step A: Stack vs Heap ----------
fn stack_vs_heap_demo() {
    // STACK: Copy types (like i32) are copied, not moved
    let x: i32 = 5;
    let y = x; // x is still usable
    println!("Stack copy demo -> x: {x}, y: {y}");

    // HEAP: String owns heap memory. Assignment MOVES ownership.
    let s1 = String::from("hello");
    let mut s2 = s1; // move; s1 is now invalid
    println!("Move demo -> s2 now owns: {s2}");

    // If we need two independent owners, clone:
    let mut s3 = s2.clone();
    println!("Clone demo -> s2: {s2}, s3: {s3}");

    // Uncommenting below would fail (s1 was moved):
    // println!("{}", s1);

    // If we update s2, s3 is not affected
    s2.push_str(" rust programming");
    println!("s2 updated -> s2: {s2}, s3: {s3}");

    // If we update s3, s2 is not affected
    s3.push_str(" baby");
    println!("s3 updated -> s2: {s2}, s3: {s3}");
}

// ---------- Step B: Ownership vs Borrowing ----------
fn takes_ownership(s: String) -> usize {
    // we own s; it will be dropped at end of function
    s.len()
}

fn borrows_immutably(s: &String) -> usize {
    // we just read; caller keeps ownership
    s.len()
}

fn borrows_mutably(s: &mut String) {
    // we can modify through &mut
    s.push_str("!");
}

// Return a slice (&str) referencing part of the original string.
// (For simplicity we take a prefix; safe for ASCII.)
fn first_n_ascii(s: &str, n: usize) -> &str {
    let end = n.min(s.len());
    &s[..end]
}

// ---------- Step C: Mini Notes helpers ----------
fn add_note(notes: &mut Vec<Note>, title: &str, body: &str) {
    notes.push(Note {
        title: title.to_string(),
        body: body.to_string(),
    });
}

// Takes ownership of a Note, transforms it, and returns it.
fn shout(mut note: Note) -> Note {
    note.title = note.title.to_uppercase();
    note.body = note.body.to_uppercase();
    note
}

// Borrows a Note and returns a new String (caller owns the new String)
fn summarize(note: &Note) -> String {
    format!("{} — {}...", note.title, first_n_ascii(&note.body, 20))
}

fn main() {
    println!("--- Stack vs Heap ---");
    stack_vs_heap_demo();

    println!("\n--- Ownership & Borrowing ---");
    let s = String::from("hello");

    // Immutable borrow
    let len1 = borrows_immutably(&s);
    println!("len(s) via & borrow = {len1}");

    // Move (we clone so we can still use s afterward)
    let len2 = takes_ownership(s.clone());
    println!("len(s) via move (on a clone) = {len2}");
    println!("still can use s after clone -> s = {}", s);

    // Mutable borrow
    let mut t = String::from("edit me");
    borrows_mutably(&mut t);
    println!("after &mut borrow -> t = {}", t);

    println!("\n--- Notes mini-project ---");
    let mut notes: Vec<Note> = Vec::new(); // Vec lives on stack, elements on heap
    add_note(&mut notes, "groceries", "milk, eggs, bread");
    add_note(&mut notes, "idea", "build a rust CLI");

    // Immutable borrows while printing summaries
    for n in &notes {
        println!("• {}", summarize(n));
    }

    println!("\nMove a Note into shout() and get it back:");
    let first = notes.remove(0); // move the Note out of the Vec
    let first = shout(first); // transform and get ownership back
    println!("after shout -> {}", summarize(&first));
    notes.insert(0, first); // move it back into the Vec

    println!("\nBorrowing rules in action:");
    {
        // Many immutable borrows at once are OK
        let titles: Vec<&str> = notes.iter().map(|n| n.title.as_str()).collect();
        println!("titles (immutable borrows): {:?}", titles);
    } // immutable borrows end here

    // Now we can take a mutable borrow
    let note0 = &mut notes[0];
    note0.body.push_str("!!!");
    println!("mutated first note: {}", summarize(note0));

    println!("\n--- Explicit heap with Box ---");
    let answer = Box::new(42);
    println!("Box<i32> lives on heap, value = {}", *answer);

    // Uncomment to see borrow checker complaints:
    // let r1 = &notes[0];        // immutable borrow
    // let r2 = &mut notes[0];    // mutable borrow at same time -> ❌
    // println!("{:?}{:?}", r1.title, r2.title);
}
