use baz;

#[test]
fn it_works() {
    let result = baz::substract(2, 2);
    assert_ne!(result, 4);
    assert_eq!(result, 0);
}
