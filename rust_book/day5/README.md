### Day 5: The Module System in Rust

Today, I dove deep into Rust's module system, which organizes code into modular namespaces, facilitating better organization, code reuse, and encapsulation.

#### Modules: Organizing Code

Modules allow for organizing related code together, effectively creating a namespace.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    mod serving {
        fn take_order() {}
    }
}
```

#### Visibility Rules

By default, items in modules are private, ensuring internal implementation details remain hidden. Using the `pub` keyword makes them accessible from outside modules.

#### Accessing Functions in Modules

You can access functions inside a module by specifying its path, either using an absolute path or a relative path.

```rust
// Absolute path
crate::front_of_house::hosting::add_to_waitlist();
// Relative path
front_of_house::hosting::add_to_waitlist();
```

#### Module Tree Analogy

It's beneficial to think of the module system as a file system:
- `mod` is analogous to a folder.
- `fn` is analogous to a file within that folder.

The module tree:
```
crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment
```

#### The `use` Keyword

To avoid typing long module paths, you can use the `use` keyword to bring a module into scope.

```rust
use crate::front_of_house::hosting;
```

#### Parent Module with `super`

The `super` keyword helps in accessing the parent module, aiding in calling functions from an outer scope.

```rust
mod back_of_house {
    fn fix_incorrect_order() {
        super::serve_order();
    }
}
```

This day was insightful as I understood the modular organization of Rust code. The module system, combined with visibility rules, ensures encapsulated and readable code structures.

