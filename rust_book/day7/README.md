### Day 7: Rust Error Handling Techniques

Today, I delved deeper into Rust's error handling mechanisms. Error handling in Rust is a bit different from many other languages; instead of using exceptions, Rust uses a combination of the `Result<T, E>` type and the `panic!` macro. I've organized the day's exploration into a dedicated module, `error_handling`, within the `errors.rs` file for easy referencing.

#### Panic!

Using `panic!` makes your program stop execution immediately.

- **Immediate Panic**: Demonstrates the immediate use of the `panic!` macro which, when called, will halt the program and print "crash and burn".
  
```rust
pub mod error_handling {
    pub fn part1() {
        // ... code here ...
    }
}
```

#### Propagating Errors

Shows how you can return the error to the calling code so it can decide what to do.

- **Function Chaining Panic**: Chaining of function calls leading to a panic scenario.
- **File Operations**: Tackling potential errors when trying to open a file or when it's not found.

```rust
pub mod error_handling {
    // ... functions part2() and part3() here ...
}
```

#### Handling Recovery

Rust gives us tools to handle recoverable errors in a more structured way.

- **Using `unwrap_or_else`**: Error handling using closures for more specific error reactions.
- **Using `unwrap` and `expect`**: Methods to directly handle the `Result` variants.

```rust
pub mod error_handling {
    // ... function part4() here ...
}
```

#### ? Operator

A concise way to handle errors that propagate up.

- **File Read Operations**: Demonstrates the use of `?` for more concise error handling in file operations.

```rust
pub mod error_handling {
    // ... function part5() here ...
}
```

#### Unwrap Examples

Shows how `unwrap` can be used for direct error handling.

- **Parsing IP Address**: Demonstrates the use of `parse().unwrap()` in the context of IP address parsing.

```rust
pub mod error_handling {
    // ... function part6() here ...
}
```

### How to Run:

After navigating to the project directory:

1. Import the `error_handling` module in the `main.rs`.
2. Call the desired function from the `error_handling` module.
3. Run using `cargo run`.

Project Structure:

```
day7
│
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs          # Main entry point
    └── errors.rs        # Module containing error handling explorations
```

Through today's exploration, I've gained a deeper appreciation for Rust's unique approach to error handling. The language's design truly places a premium on safety and expressiveness.