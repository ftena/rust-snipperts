# rust-snipperts
Rust snippets

## More about Formatted print
https://doc.rust-lang.org/rust-by-example/hello/print.html

## More about Tuples, Arrays and Slices
https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html

https://doc.rust-lang.org/stable/rust-by-example/primitives/array.html

## More about match, destructuring in a match block (i.e. tuples, arrays, enums, pointers/ref and structs), guards and binding
https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html

## More about Closures
https://doc.rust-lang.org/stable/rust-by-example/fn/closures.html

- As input parameters: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_parameters.html
- As output parameters: https://doc.rust-lang.org/stable/rust-by-example/fn/closures/output_parameters.html
- Example in std:
    - Using Iterator::any, iter() and into_iter(): https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples/iter_any.html
    - Using Iterator::find, iter() and into_iter(): https://doc.rust-lang.org/stable/rust-by-example/fn/closures/closure_examples/iter_find.html
- Diverging functions: https://doc.rust-lang.org/stable/rust-by-example/fn/diverging.html

## More info about modules
https://doc.rust-lang.org/stable/rust-by-example/mod.html

- Visibility: https://doc.rust-lang.org/stable/rust-by-example/mod/visibility.html
- `use` declaration: https://doc.rust-lang.org/stable/rust-by-example/mod/use.html
- `super` and `self`: https://doc.rust-lang.org/stable/rust-by-example/mod/super.html

Modules can be mapped to a file/directory hierarchy. More info @ https://doc.rust-lang.org/stable/rust-by-example/mod/split.html

## Crates
https://doc.rust-lang.org/stable/rust-by-example/crates.html

A crate is a compilation unit in Rust. Whenever `rustc some_file.rs` is called, `some_file.rs` is treated as the crate file. If `some_file.rs` has `mod` declarations in it, then the contents of the module files would be inserted in places where `mod` declarations in the crate file are found, before running the compiler over it. In other words, modules do not get compiled individually, only crates get compiled.

A crate can be compiled into a binary or into a library. By default, `rustc` will produce a binary from a crate. This behavior can be overridden by passing the `--crate-type` flag to `lib`.

## Cargo
https://doc.rust-lang.org/stable/rust-by-example/cargo/deps.html
https://doc.rust-lang.org/cargo/reference/cargo-targets.html

- Library: The filename defaults to `src/lib.rs`, and the name of the library defaults to the name of the package. A package can have only one library.
- Binaries: The default binary filename is `src/main.rs`, which defaults to the name of the package. Additional binaries are stored in the `src/bin/` directory.

