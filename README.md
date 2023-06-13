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

- As input parameters (+ `Fn`, `FnMut` and `FnOnce`): https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_parameters.html
- As output parameters (use `impl Trait` to return them): https://doc.rust-lang.org/stable/rust-by-example/fn/closures/output_parameters.html
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

## Attributes
https://doc.rust-lang.org/stable/rust-by-example/attribute.html

When attributes apply to a whole crate, their syntax is `#![crate_attribute]`, and when they apply to a module or item, the syntax is `#[item_attribute]` (notice the missing bang `!`).

## Generics
https://doc.rust-lang.org/stable/rust-by-example/generics.html

A type parameter is specified as generic by the use of angle brackets and upper camel case: <Aaa, Bbb, ...>. "Generic type parameters" are typically represented as <T>.

- Generics: https://doc.rust-lang.org/stable/rust-by-example/generics.html
- Functions: https://doc.rust-lang.org/stable/rust-by-example/generics/gen_fn.html
- Implementation: https://doc.rust-lang.org/stable/rust-by-example/generics/impl.html
- Where clauses: https://doc.rust-lang.org/stable/rust-by-example/generics/where.html
- Associated items: https://doc.rust-lang.org/stable/rust-by-example/generics/assoc_items.html
- Phantom type parameters: https://doc.rust-lang.org/stable/rust-by-example/generics/phantom.html

## Scoping rules
- The notion of a destructor and `Drop` trait: https://doc.rust-lang.org/stable/rust-by-example/scope/raii.html
- Partial moves: https://doc.rust-lang.org/stable/rust-by-example/scope/move/partial_move.html
- Ref pattern: https://doc.rust-lang.org/stable/rust-by-example/scope/borrow/ref.html
- Lifetimes:
    - Explicit annotation: the borrow checker uses explicit lifetime annotations to determine how long references should be valid.- https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/explicit.html
    - Bounds: lifetimes (themselves generic) use bounds as well.- https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_bounds.html
    - Coercion: a longer lifetime can be coerced into a shorter one so that it works inside a scope it normally wouldn't work in.- https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/lifetime_coercion.html
    - Static: Rust has a few reserved lifetime names. One of those is 'static. You might encounter it in two situations: reference lifetime or trait bound.- https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime/static_lifetime.html 

## Traits
- Returning Traits with `dyn`: https://doc.rust-lang.org/stable/rust-by-example/trait/dyn.html#returning-traits-with-dyn
- Operator Overloading: https://doc.rust-lang.org/stable/rust-by-example/trait/ops.html
- Iterators: https://doc.rust-lang.org/stable/rust-by-example/trait/iter.html
- `impl Trait`: https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html
- Supertraits: https://doc.rust-lang.org/stable/rust-by-example/trait/supertraits.html

## macro_rules!
- Overload: https://doc.rust-lang.org/stable/rust-by-example/macros/overload.html
- Repeat: Macros can use + in the argument list to indicate that an argument may repeat at least once, or *, to indicate that the argument may repeat zero or more times.- https://doc.rust-lang.org/stable/rust-by-example/macros/repeat.html
- DRY: Macros allow writing DRY code by factoring out the common parts of functions and/or test suites.- https://doc.rust-lang.org/stable/rust-by-example/macros/dry.html
- DSL: A DSL is a mini "language" embedded in a Rust macro. It is completely valid Rust because the macro system expands into normal Rust constructs, but it looks like a small language. This allows you to define concise or intuitive syntax for some special functionality (within bounds).- https://doc.rust-lang.org/stable/rust-by-example/macros/dsl.html
- Variadics: A variadic interface takes an arbitrary number of arguments. For example, `println!` can take an arbitrary number of arguments, as determined by the format string.- https://doc.rust-lang.org/stable/rust-by-example/macros/variadics.html

## Error handling
https://doc.rust-lang.org/stable/rust-by-example/error.html

- An explicit `panic` is mainly useful for tests and dealing with unrecoverable errors. For prototyping it can be useful, for example when dealing with functions that haven't been implemented yet, but in those cases the more descriptive `unimplemented` is better. In tests `panic` is a reasonable way to explicitly fail.
- The `Option` type is for when a value is optional or when the lack of a value is not an error condition.
- When there is a chance that things do go wrong and the caller has to deal with the problem, use Result.
