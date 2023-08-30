### Day 6: Rust Common Collections - Vectors, Strings, and HashMaps

Today, I delved into three of Rust's standard library collection types: vectors, strings, and hashmaps. Using Rust's modular system, I organized my exploration in separate modules to ensure clarity and ease of reference.

#### Vectors

Vectors are dynamic arrays that can grow or shrink in size.

- **Vectors Creation**: Different ways to create vectors, using `vec!` macro or initializing with `Vec::new()`.
- **Vector Access**: Accessing elements in a vector using indexing or the `get()` method.
- **Iteration**: Iterating over the values in a vector.
- **Enum in Vectors**: Demonstrating the storage of different types in a vector using Rust's enum.

```rust
pub mod vectors {
    // ... code here ...
}
```

#### Strings

Strings are UTF-8 encoded collection of characters.

- **String Creation**: Creating new strings from string literals or using `String::from`.
- **Updating Strings**: Appending to a string using `push_str` or `push`.
- **String Concatenation**: Joining strings.
- **UTF-8 and Indexing**: An exploration into why Rust strings don't support indexing.

```rust
pub mod strings {
    // ... code here ...
}
```

#### HashMaps

HashMaps store values by a key. Each key maps to a value.

- **HashMap Creation**: Demonstrating how to create hashmaps and insert values.
- **Access**: Accessing values using a key with `get`.
- **Entry API**: Using the `entry` method to only insert a value if the key has no value.
- **Word Count Example**: Creating a word frequency counter.

```rust
pub mod hashmaps {
    // ... code here ...
}
```

### How to Run:

After navigating to the project directory:

1. Uncomment the desired function calls in the `main.rs`.
2. Run using `cargo run`.

Project Structure:

```
day6
│
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs      # Main entry point
    └── collections.rs      # Module containing vectors, strings, and hashmap explorations
```

Today's exploration deepened my understanding of how to work with Rust's common collections and their associated methods and patterns. As always, Rust's focus on safety and expressiveness shined through.

