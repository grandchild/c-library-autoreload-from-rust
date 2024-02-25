# Dynamic-Library Autoreload in Rust


## What is this?

An example of how to auto-reload a dynamic C library from Rust whenever the
underlying C library file changes. This is immensely useful for rapid
prototyping and short debug cycles.

It contains a simple C library and a Rust application that uses it. The library
is reloaded whenever it's recompiled, and the Rust application continues to use
the new library functions without needing to be restarted.


### Initial Situation
- A dynamically-linked library (`.so` or `.dll` or `.dylib` file)
- The library has a C API/ABI
- A Rust application using said API
- The _library_ is under development
- The feedback cycle from library changes is too long, i.e. restarting the app
  often is manual & cumbersome

### What We Want
- The Rust application should continuously run library code
- When the library file changes, the library is reloaded
- The Rust application should not need to be restarted
- The Rust application should not need to be recompiled

### The Plan
- Load the library dynamically
- Check mtime of the library file every second
- If the mtime has changed, reload the library which updates the function
  pointers
- Resume using the library functions

### How It's Done
- Use either [`libloading`](https://crates.io/crates/libloading) or the two
  available APIs of [`dlopen2`](https://crates.io/crates/dlopen2) to load the
  library dynamically
- Use `std::fs::metadata` to get the mtime of the library file
- Use `std::thread::sleep` to wait for a second
- Repeat


## Running

Build the library:

```sh
cmake -B build
cmake --build build
```

Run the Rust application:

```sh
cargo run
```

Now you can edit [src/lib.c](src/lib.c) and re-run `cmake --build build` to see
the changes reflected in the running Rust application:


```sh
# In Rust Land                            # In C Land

$ cargo run | ts -s
  ...
00:00:03 foo() -> 4
00:00:03 bar() is not defined         # Editing to change `foo()` and add `bar()`
00:00:04 foo() -> 4                   $ cmake --build build
00:00:04 bar() is not defined         [ 50%] Building C object CMakeFiles/foo.dir/src/lib.c.o
00:00:05 foo() -> 4                   [100%] Linking C shared library libfoo.so
00:00:05 bar() is not defined         [100%] Built target foo
00:00:06 Reloading "build/libfoo.so"
00:00:06 foo() -> 5
00:00:06 bar() -> 10
00:00:07 foo() -> 5
00:00:07 bar() -> 10
  ...
```

## Inspect

- [CMakeLists.txt](CMakeLists.txt) - The manifest for the C library
- [src/lib.c](src/lib.c) - The C library

- [Cargo.toml](Cargo.toml) - The manifest for the Rust application
- [src/main.rs](src/main.rs) - The Rust application

In the Rust application there are three similar sections, the first for
`libloading`, the other two for `dlopen2`'s two APIs. Only one of the sections
may be uncommented at a time. Use the `//*`/`/*` trick at the beginning of
a section to toggle it with a single character.


## Caveats

- The C library functions are exported via `__attribute__` which probably only
  works out of the box in GCC/Clang.
- Tested only with GCC on Linux.

## License

CC0
