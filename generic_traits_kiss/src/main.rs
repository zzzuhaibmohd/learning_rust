// Step 1: Generics + inherent methods (no trait bounds yet)

#[derive(Debug, Clone)]
struct Boxy<T> {
    inner: T,
}

impl<T> Boxy<T> {
    fn new(value: T) -> Self {
        Self { inner: value }
    }

    // Borrow the inner value (so we don't move it out)
    fn value(&self) -> &T {
        &self.inner
    }

    // Swap in a new value and get the old value back
    fn replace(&mut self, value: T) -> T {
        std::mem::replace(&mut self.inner, value)
    }
}

fn main() {
    println!("== Step 1: Generic struct + methods ==");

    println!("Int");
    let mut n = Boxy::new(42);
    println!("n = {:?}", n.value()); // &i32
    let old = n.replace(100);
    println!("old: {old}, now: {:?}", n.value());

    println!("String");
    let mut s = Boxy::new("hello");
    println!("s = {:?}", s.value()); // &&str, prints fine with Debug
    s.replace("world");
    println!("s = {:?}", s.value()); // &&str, prints fine with Debug

    println!("Bool");
    let mut b = Boxy::new(true);
    println!("b = {:?}", b.value()); // &bool, prints fine with Debug
    b.replace(false);
    println!("b = {:?}", b.value()); // &bool, prints fine with Debug
}
