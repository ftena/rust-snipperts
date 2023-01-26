fn main() {
    let mut result = bar::add(1 , 2);

    println!("result after calling bar::add(1 , 2): {}", result);

    // https://doc.rust-lang.org/stable/rust-by-example/types/cast.html
    result = baz::substract(1, 2) as usize;

    println!("weird result after calling baz::substract(1 , 2) as usize: {}", result);
    
    println!("result after calling baz::substract(1 , 2): {}", baz::substract(1, 2));

}
