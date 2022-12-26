#[warn(unused_variables)]
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumberv1(i32);

#[derive(Debug, PartialEq)]
struct EvenNumberv2(i32);

impl TryFrom<i32> for EvenNumberv1 {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumberv1(value))
        } else {
            Err(())
        }
    }
}

impl TryFrom<i32> for EvenNumberv2 {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumberv2(value))
        } else {
            Err("Only values greater than zero!")
        }
    }
}

fn main() {
    // v1
    // TryFrom
    assert_eq!(EvenNumberv1::try_from(8), Ok(EvenNumberv1(8)));
    assert_eq!(EvenNumberv1::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumberv1, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumberv1(8)));
    let result: Result<EvenNumberv1, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // v2
    // TryFrom
    assert_eq!(EvenNumberv2::try_from(8), Ok(EvenNumberv2(8)));
    assert_eq!(EvenNumberv2::try_from(5), Err("Only values greater than zero!"));

    // TryInto
    let result: Result<EvenNumberv2, &str> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumberv2(8)));
    let result: Result<EvenNumberv2, &str> = 5i32.try_into();
    assert_eq!(result, Err("Only values greater than zero!"));
}
