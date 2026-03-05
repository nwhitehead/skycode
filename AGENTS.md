# AGENTS.md - Agentic Coding Guidelines

This file provides guidelines for AI agents working on the CritterCoder codebase.

## Project Overview

CritterCoder is an open-source AI coding agent harness (TUI application) built with Rust using ratatui for the terminal UI.

## Build, Lint, and Test Commands

### Building
```bash
# Build the project
cargo build

# Build with optimizations
cargo build --release

# Build a specific example
cargo build --example input_test
```

### Running
```bash
# Run the main application
cargo run

# Run a specific example
cargo run --example input_test
```

### Testing
```bash
# Run all tests
cargo test

# Run a single test by name
cargo test <test_name>

# Run tests with output display
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

### Linting and Formatting
```bash
# Run clippy for linting
cargo clippy

# Run clippy with all warnings (including denied)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check

# Run clippy + fmt in one command
cargo fmt && cargo clippy
```

### Other Useful Commands
```bash
# Check dependencies for updates
cargo outdated

# Generate documentation
cargo doc --no-deps

# View documentation locally
cargo doc --no-deps --open

# Clean build artifacts
cargo clean
```

## Code Style Guidelines

### General Principles

- Follow standard Rust conventions (the "Rustic" style)
- Keep lines under 100 characters when reasonable
- Use meaningful, descriptive names

### Imports

- Use absolute paths with `crate::` for internal modules
- Group imports by crate: standard library first, then external crates, then internal
- Use nested imports for related items from the same crate:
  ```rust
  use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
  use ratatui::prelude::*;
  ```
- Avoid wildcard imports (`use foo::*`) except for `prelude` modules

### Formatting

- Use 4 spaces for indentation (Rust default)
- Place `use` statements at the top of the file, below module-level comments/docstrings
- Use trailing commas in multi-line collections
- Format match arms with block format for complex arms:
  ```rust
  match value {
      Pattern1 => expr,
      Pattern2 => {
          // multi-line body
      }
  }
  ```

### Types

- Use explicit type annotations where it improves readability
- Prefer `&str` over `String` for function parameters when borrowing is sufficient
- Use `Self` in impl blocks
- Prefer returning `Result<T, E>` for fallible operations
- Use `io::Result<()>` for main functions that can fail with I/O errors

### Naming Conventions

- **Snake_case** for variables, functions, and modules: `my_function`, `my_var`
- **PascalCase** for types and traits: `MyStruct`, `MyTrait`
- **SCREAMING_SNAKE_CASE** for constants: `MAX_SIZE`
- Prefix unused variables with underscore: `_unused_var`
- Use descriptive names: `user_input` not `ui`, `connection_status` not `cs`

### Error Handling

- Use `Result` for recoverable errors
- Use `expect()` or `unwrap()` only when the error is truly unrecoverable (e.g., in examples or prototypes)
- Provide meaningful error messages in `expect()` calls:
  ```rust
  // Good
  let reader = ImageReader::new(data).expect("Failed to create image reader");
  
  // Avoid
  let reader = ImageReader::new(data).unwrap();
  ```
- Use `?` operator for propagating errors
- For main/error-main, use `Box<dyn std::error::Error>` for flexible error handling

### Structs and Impl Blocks

- Use `#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]` as appropriate
- Implement `Widget` trait for ratatui widgets using the `&self` pattern:
  ```rust
  impl Widget for &MyWidget {
      fn render(self, area: Rect, buf: &mut Buffer) {
          // rendering logic
      }
  }
  ```
- Use `pub struct` for public types, `struct` for internal types
- Prefer builder pattern or `new()` constructor functions over complex `new()` with many parameters

### Documentation

- Add docstrings (`///`) to public APIs
- Use doc comments for module-level documentation
- Document widget behavior for UI components

### Testing

- Place unit tests in the same file using `#[cfg(test)]` module
- Place integration tests in `tests/` directory
- Use descriptive test names: `test_name_describes_what_is_verified`
- Use `#[test]` attribute for unit tests

### Ratatui/TUI Specific

- Use `ratatui::prelude::*` for common imports
- Create custom widget implementations using the `Widget` trait
- Use `Block`, `Borders`, `Padding` for container widgets
- Handle resize events gracefully
- Support both mouse and keyboard input

### Git Conventions

- Make one logical change per commit
- Write commit messages in imperative mood: "Add feature" not "Added feature"
- Keep the first line under 72 characters
