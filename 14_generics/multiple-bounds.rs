// more info @ https://doc.rust-lang.org/stable/rust-by-example/generics/multi_bounds.html

use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_only_debug<T: Debug>(t: &T) {
    println!("Only debug: `{:?}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u);
}

fn main() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_only_debug(&vec);
    //compare_prints(&vec); // `std::fmt::Display` is not implemented for `Vec<{integer}>
    //compare_prints(&array); // the trait `std::fmt::Display` is not implemented
    // TODO ^ Try uncommenting this.

    compare_types(&array, &vec);
}

