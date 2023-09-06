## Day 8: Rust Traits and Lifetimes

### Directory Structure

```
day8/
│
├── day8_1/
│   ├── src/
│   │   ├── main.rs
│   │   └── traits.rs
│   ├── Cargo.lock
│   └── Cargo.toml
│
├── day8_2/
│   ├── src/
│   │   ├── lifetimes.rs
│   │   └── main.rs
│   ├── Cargo.lock
│   └── Cargo.toml
│
└── README.md
```

### Lifetimes

#### Definition

In Rust, lifetimes ensure that references are valid for the duration of their 
usage. They provide safety from dangling references.

#### Examples

- **Simple Lifetime Example**: Shows basic usage of lifetimes and how they 
ensure memory safety. 

- **Shared Lifetimes**: Demonstrates that the return value of a function can 
have the same lifetime as its parameters.

- **Invalid References**: Showcases an example where references become invalid 
once the borrowed value goes out of scope.

- **Lifetime Structs**: Displays how structs can use lifetimes to contain 
references.

- **Lifetime Elision**: Discusses the rules and practices around omitting 
lifetimes when they can be inferred.

- **Generic Lifetimes**: Demonstrates how lifetimes can be paired with generics.

### Traits

#### Definition

Traits are a way to group method signatures together to define a set of 
behaviors necessary for certain types.

#### Examples

- **Tweet & NewsArticle**: Introduces two struct types, `Tweet` and 
`NewsArticle`.

- **Summary Trait**: A trait called `Summary` with methods `summarize` and 
`summarize_author`. Also contains a default method implementation.

- **Display Trait Implementation**: Demonstrates how to implement the standard 
`Display` trait for the `Tweet` struct.

- **Trait Bounds**: Shows how to enforce that function arguments implement 
specific traits.

- **Multiple Trait Bounds**: Demonstrates the usage of multiple trait bounds in 
function parameters.

- **Return Types with Traits**: How to return types that implement specific 
traits.

- **Generic Type Traits**: Displays how to implement methods on generic types 
that also require certain traits.

---

This README gives an overview of the day's content. Each subsection can be 
expanded upon with more details, explanations, or code examples from 
`lifetimes.rs` and `traits.rs`.
