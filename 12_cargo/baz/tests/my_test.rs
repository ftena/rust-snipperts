/*
 * Integration tests go in a different directory, they don’t need the #[cfg(test)] annotation.
 * More info @ https://doc.rust-lang.org/book/ch11-03-test-organization.html
 */

use baz;

#[test]
fn it_works() {
    let result = baz::substract(2, 2);
    assert_ne!(result, 4);
    assert_eq!(result, 0);
}
