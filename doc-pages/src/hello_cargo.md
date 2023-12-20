# hello\_cargo

The book section dealing with this is [Section 1.3](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html).

---

This is the same program as in [hello\_world](hello_world.md) but the project folder is created and managed using `cargo`. 

- `cargo` is the official Rust build system and package manager.
- To create a new project run `cargo new PROJECT_NAME`.
    - This creates a new folder `PROJECT_NAME` with the following structure:
      ```bash
      PROJECT_NAME/
        |-- .git/ # (...)
        |-- .gitignore
        |-- Cargo.toml
        |-- src/
            |-- main.rs # With an initial simple 'Hello, world!' program
      ```
      The new Git repository and .gitignore file won't be created if `cargo` is run inside project which is already initialised with Git.
- The section `[dependencies]` in the `Cargo.toml` file can be used to add any dependencies the project uses.
- Main `cargo` commands:
    - `cargo build` is used to build the project.
        - Saves the build results in the *target/debug/* directory.
    - `cargo run` is used to run the project. This will also build it if any changes have been made since last project build.
    - `cargo check` checks that the project compiles. Useful if the project is large and compile times are therefore long and you just want to check if the code compiles.

## The full code

File: **main.rs**
```rust
{{#include ../../hello_cargo/src/main.rs}}
```
