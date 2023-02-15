// more info @ https://doc.rust-lang.org/stable/rust-by-example/generics/bounds.html

// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
fn printer<T: fmt::Display>(t: T) {
    println!("{}", t);
}

// Bounding restricts the generic to types that conform to the bounds. That is:
struct S<T: fmt::Display>(T);

fn main() {
    // Error! `Vec<T>` does not implement `Display`. This
    // specialization will fail.
    //let s = S(vec![1]);
}
