### Day 4: Enumerations and Pattern Matching

Today, I explored the powerful concept of enumerations (enums) and how to use pattern matching in Rust.

#### Enumerations (Enums)

I created various enums like `IPAddrKind`, which can take either an IPv4 or IPv6 address, and `Message`, which defines different message types.

```rust
enum IPAddrKind {
    V4(String),
    V6(String),
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}
```

#### Associated Functions and Methods

I explored the difference between associated functions and methods. Methods are defined within the context of a struct, while associated functions are defined within the context of an enum.

```rust
impl Message{
    fn some_function(){
        println!("Learning Rust!")
    }
}
```

#### Option Enum

I worked with the `Option` enum, a powerful tool in Rust for handling the presence and absence of values.

```rust
let some_number = Some(5);
let absent_number: Option<i32> = None;
```

#### Pattern Matching

Pattern matching provides control over a program's flow based on the value of expressions. I used the `match` expression to handle various cases with the `Coin` and `UsState` enums.

```rust
fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}
```

#### Unwrapping Options

I learned how to unwrap an option to obtain the inner value using the `unwrap()` method.

```rust
let sum = x + y.unwrap();
```

#### `if let` Syntax

I also discovered the `if let` syntax, which provides a more concise way to handle values that match one pattern and ignore the rest.

```rust
if let Some(3) = some_value{
    println!("three");
}
```

This day helped me understand the utility and elegance of enums and pattern matching in Rust, enriching my toolbox of programming constructs.
