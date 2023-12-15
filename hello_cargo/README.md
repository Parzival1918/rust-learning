# cargo\_hello

- Create a new project with *cargo* using the command `cargo new NAME`.
    - Creates a `Cargo.toml` and `src/main.rs` folder and file.
    - Also initializes a git repository, unless *cargo* command is used within a git repo.
- Build the project using `cargo build`. Creates an executable in *target/debug/hello_cargo*.
    - When running this command for the first time it also creates a `Cargo.lock` file. Never will have to touch this file manually, it is managed by *cargo*.
- Run the project using `cargo run`. It will build it if any changes have been done since last modification.
- `cargo check` can be used to check if the project compiles but does not actually do the compilation.
