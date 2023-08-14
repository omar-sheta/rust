# My Rust Learning Journey

This repository documents my progress as I learn Rust, a programming language focused on performance, concurrency, and memory safety.

## What I've Learned So Far

### Ownership and Borrowing

- **Ownership Rules**: Rust's ownership rules ensure that memory is safely and efficiently managed.
- **Borrowing**: I learned how to use borrowing, both mutable and immutable, to access data without taking ownership.
- **Shallow Copy vs Deep Copy**: Through experimenting with copying and cloning, I understood how data is handled differently with the `Copy` trait.

### Functions and Returning Values

- **Passing Values**: Explored how ownership can be transferred in function calls, and how the `Copy` trait affects this behavior.
- **Returning Values**: Learned how to return values and ownership from functions.

### Mutable and Immutable References

- **Immutable References**: These allow read-only access to data.
- **Mutable References**: Allow modification of the data they point to, but there can only be one mutable reference to a particular piece of data in a particular scope.
- **Dangling References**: Learned about the concept of dangling references and why they are prevented in Rust.

### Code Examples

- `test1()`: Experimenting with ownership, cloning, and copying.
- `test2()`: Understanding how ownership can be given and taken back in functions.
- `test3()`: Exploring mutable and immutable references and how they affect data modification.
- `test4()`: Delving into the rules of references, such as having either one mutable reference or any number of immutable references.

## Reflections

I find Rust's approach to memory management and type safety both novel and intriguing. While the learning curve has been somewhat steep, the concepts are starting to make sense, and I can see the benefits in terms of performance and safety.

## How to Run the Code

1. Save the code in a `.rs` file.
2. Open a terminal and navigate to the directory containing the file.
3. Compile the code using the command:

   ```sh
   rustc filename.rs
   ```

4. Run the compiled program:

   ```sh
   ./filename
   ```

Replace `filename` with the actual name of the file.

## Next Steps

- Explore error handling in Rust.
- Learn about Rust's concurrency features.
- Work on a small project to apply what I've learned.

---