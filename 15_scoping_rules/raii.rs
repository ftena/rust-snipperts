// more info @ https://doc.rust-lang.org/stable/rust-by-example/scope/raii.html
// The notion of a destructor in Rust is provided through the Drop trait.
// The destructor is called when the resource goes out of scope.
// This trait is not required to be implemented for every type,
// only implement it for your type if you require its own destructor logic.

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}

