// more info @ https://doc.rust-lang.org/stable/rust-by-example/trait/derive.html

// This is a similar example as 3.1_structures/structures.rs but
// using the `#[derive]` attribute to allow the struct copy.

// A struct with two fields
// We can derive a `Copy` implementation. `Clone` is also required, as it's
// a supertrait of `Copy`.
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Clone, Copy)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// It calculates the area of a Rectangle
fn rect_area(r: Rectangle) -> f32 {
    // Destructure the point using a `let` binding
    let Point { x: x1, y: y1 } = r.top_left;
    let Point { x: x2, y: y2 } = r.bottom_right;
    
    let width = (x2 - x1).abs();
    let height = (y2 - y1).abs();
    
    width * height
}

fn main() {
    // New rectangle to calculate its area
    let rectangle = Rectangle {
        top_left: Point { x: 5.0, y: 5.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };
    
    println!("rect_area: {}", rect_area(rectangle));

    // No error! rectangle can be copied!
    println!("rect_area: {}", rect_area(rectangle));
}

