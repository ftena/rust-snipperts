pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/*
 * The #[cfg(test)] annotation on the tests module tells Rust to compile
 * and run the test code only when you run cargo test,
 * not when you run cargo build.
 * More info @ https://doc.rust-lang.org/book/ch11-03-test-organization.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
