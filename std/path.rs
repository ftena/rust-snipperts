use std::path::Path;

fn main() {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Display`able structure
    let _display = path.display();

    println!("_display is {}", _display);

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    let mut new_path = path.join("..");

    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("LICENSE");

    // `set_file_name` updates the file name of the `PathBuf`
    new_path.set_file_name("README.md");

    // Convert the `PathBuf` into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

