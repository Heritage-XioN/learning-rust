# Learning Rust

A comprehensive reference guide for Rust developers — from first steps to production patterns.

---

## Table of Contents

1. [Getting Started](#1-getting-started)
2. [Cargo & Tooling](#2-cargo--tooling)
3. [Project Structure](#3-project-structure)
4. [Variables & Constants](#4-variables--constants)
5. [Data Types](#5-data-types)
6. [Functions](#6-functions)
7. [Control Flow](#7-control-flow)
8. [Structs, Enums & Pattern Matching](#8-structs-enums--pattern-matching)
9. [Collections](#9-collections)
10. [Traits & Generics](#10-traits--generics)
11. [Ownership & Borrowing](#11-ownership--borrowing)
12. [Lifetimes](#12-lifetimes)
13. [Error Handling](#13-error-handling)
14. [Closures & Iterators](#14-closures--iterators)
15. [Smart Pointers](#15-smart-pointers)
16. [Concurrency](#16-concurrency)
17. [Macros](#17-macros)
18. [Testing](#18-testing)
19. [Unsafe Rust](#19-unsafe-rust)
20. [Useful Crates & Ecosystem](#20-useful-crates--ecosystem)

---

## 1. Getting Started

### Install Rust
The recommended way to install Rust is via [rustup](https://rustup.rs):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Manage Toolchains
```bash
rustup update                  # update Rust to the latest version
rustup default stable          # use the stable toolchain
rustup default nightly         # switch to nightly (for experimental features)
rustup show                    # show installed toolchains and active one
rustup component add clippy    # add a component (linter)
rustup target add wasm32-unknown-unknown  # add a compilation target
```

### Rust Editions
Rust uses **editions** (2015, 2018, 2021, 2024) to introduce breaking changes without breaking old code. Each project specifies its edition in `Cargo.toml`:
```toml
[package]
edition = "2024"
```

### Hello World
```rust
fn main() {
    println!("Hello, world!");
}
```
Compile and run manually (without Cargo):
```bash
rustc main.rs    # compile
./main           # run the binary
```

---

## 2. Cargo & Tooling

Cargo is Rust's build system **and** package manager. Almost everything goes through it.

### Creating Projects
```bash
cargo new my_app          # create a new binary project (with git init)
cargo new my_lib --lib    # create a new library project
cargo init                # initialize a project in the current directory
cargo init --lib          # initialize a library in the current directory
```

### Building & Running
```bash
cargo build               # compile in debug mode (target/debug/)
cargo build --release     # compile with optimizations (target/release/)
cargo run                 # build + run the binary
cargo run --release       # build + run with optimizations
cargo run -- arg1 arg2    # pass arguments to the binary
```

### Code Quality
```bash
cargo check               # type-check without producing a binary (fastest feedback)
cargo clippy              # run the Clippy linter — catches common mistakes & suggests idiomatic code
cargo fmt                 # auto-format all source files with rustfmt
cargo fmt -- --check      # check formatting without modifying files (useful in CI)
```

### Testing & Docs
```bash
cargo test                # run all unit + integration tests
cargo test test_name      # run a specific test by name
cargo test -- --nocapture # show println! output during tests
cargo bench               # run benchmarks (requires nightly or criterion)
cargo doc --open          # generate and open documentation in the browser
```

### Dependencies
```bash
cargo add serde           # add a dependency (requires cargo-edit or Rust 1.62+)
cargo add serde --features derive   # add with a feature flag
cargo add tokio -F full   # shorthand for --features
cargo remove serde        # remove a dependency
cargo update              # update dependencies to latest compatible versions
cargo tree                # display the dependency tree
```

### Useful Cargo Plugins
```bash
cargo install cargo-watch     # auto-rebuild on file changes
cargo watch -x run            # re-run on save
cargo watch -x test           # re-test on save

cargo install cargo-expand    # expand macros (see what derive macros generate)
cargo expand                  # show expanded code

cargo install cargo-audit     # check dependencies for known vulnerabilities
cargo audit                   # run the audit
```

### Cargo.toml Essentials
```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2024"
authors = ["Your Name <you@example.com>"]
description = "A short description"
license = "MIT"

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
pretty_assertions = "1"    # only used in tests

[profile.release]
opt-level = 3              # maximum optimizations
lto = true                 # link-time optimization
strip = true               # strip debug symbols from binary
```

---

## 3. Project Structure

### Small Project (single binary)
```
my_app/
├── Cargo.toml
├── src/
│   └── main.rs            # entry point
├── tests/                 # integration tests
│   └── integration_test.rs
└── README.md
```

### Medium Project (binary + modules)
```
my_app/
├── Cargo.toml
├── src/
│   ├── main.rs            # entry point — thin, delegates to lib
│   ├── lib.rs             # library root — re-exports modules
│   ├── config.rs          # configuration module
│   ├── db/                # sub-module directory
│   │   ├── mod.rs         # module declaration (or use db.rs)
│   │   ├── connection.rs
│   │   └── queries.rs
│   └── routes/
│       ├── mod.rs
│       ├── auth.rs
│       └── users.rs
├── tests/
│   └── api_tests.rs
├── benches/
│   └── benchmark.rs
└── README.md
```

### Large Project (Cargo workspace — multiple crates)
```
my_workspace/
├── Cargo.toml             # workspace root
├── crates/
│   ├── core/              # shared business logic
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── api/               # HTTP server
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   └── cli/               # command-line tool
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── README.md
```

Workspace `Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = [
    "crates/core",
    "crates/api",
    "crates/cli",
]

[workspace.dependencies]      # shared dependency versions
serde = { version = "1", features = ["derive"] }
```

### Module System — `mod`, `use`, `pub`
```rust
// src/lib.rs
pub mod config;       // loads from src/config.rs
pub mod db;           // loads from src/db/mod.rs (or src/db.rs)

// src/config.rs
pub struct AppConfig {
    pub port: u16,
    pub db_url: String,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig { port: 8080, db_url: String::from("postgres://localhost/mydb") }
    }
}

// src/main.rs
use my_app::config::AppConfig;

fn main() {
    let config = AppConfig::new();
    println!("Starting on port {}", config.port);
}
```

Visibility rules:
| Keyword         | Visibility                                    |
|-----------------|-----------------------------------------------|
| (default)       | Private to the current module                 |
| `pub`           | Public to everyone                            |
| `pub(crate)`    | Public within the current crate only          |
| `pub(super)`    | Public to the parent module only              |
| `pub(in path)`  | Public to a specific ancestor module          |

---

## 4. Variables & Constants

### Variable Bindings
```rust
let x = 5;              // immutable by default
let mut y = 10;          // mutable — can be reassigned
y += 1;

let _unused = 42;        // prefix with _ to suppress unused variable warnings
```

### Type Annotations
```rust
let a: i32 = 23;
let b: f64 = 3.14;
let c: bool = true;
let d: char = 'Z';
let e: &str = "hello";
let f: String = String::from("hello");
```

### Shadowing
Re-declaring a variable in the same scope creates a new binding (can even change the type):
```rust
let x = 5;
let x = x + 1;           // x is now 6
let x = "now a string";  // completely new variable, different type
```

### Destructuring
```rust
let (a, b, c) = (1, 2.0, "three");    // tuple destructuring
let [first, .., last] = [1, 2, 3, 4]; // array destructuring
let (x, _) = (10, 20);                // ignore a value with _
```

### Constants & Statics
```rust
// const: compile-time constant, inlined everywhere, no fixed memory address.
const MAX_POINTS: u32 = 100_000;

// static: has a fixed memory address, lives for the entire program.
// use static when you need a global reference (&'static).
static LANGUAGE: &str = "Rust";

// static mut exists but requires unsafe to access — prefer Mutex/AtomicUsize instead.
```

### Type Casting with `as`
```rust
let i: u8 = 42;
let j: u16 = i as u16;       // widening — safe
let k: u8  = 300u16 as u8;   // narrowing — wraps (k == 44)
let f: f64 = 3.99;
let n: i32 = f as i32;       // truncates decimal (n == 3)
```

### Type Aliases
```rust
type Kilometers = i32;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
```

---

## 5. Data Types

### Scalar Types

#### Integers
| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | `i8`   | `u8`     |
| 16-bit  | `i16`  | `u16`    |
| 32-bit  | `i32`  | `u32`    |
| 64-bit  | `i64`  | `u64`    |
| 128-bit | `i128` | `u128`   |
| arch    | `isize`| `usize`  |

- **Signed** — can be positive or negative
- **Unsigned** — can only be positive (or zero)
- **`isize`/`usize`** — pointer-sized, depends on architecture (64-bit on 64-bit systems)

Number literals:
```rust
let decimal     = 98_222;       // underscores for readability
let hex         = 0xff;
let octal       = 0o77;
let binary      = 0b1111_0000;
let byte        = b'A';         // u8 only
let typed       = 42u8;         // type suffix
```

#### Floats
| Length | Type  |
|--------|-------|
| 32-bit | `f32` |
| 64-bit | `f64` |

`f64` is the default. Use `f32` only when memory/performance requires it.
```rust
let x = 2.0;        // f64 (default)
let y: f32 = 3.0;   // f32
```

#### Bool
```rust
let t: bool = true;
let f: bool = false;
```

#### Char
4 bytes, represents a single Unicode scalar value:
```rust
let c: char = 'z';
let emoji: char = '🦀';
```

### String Types

#### `&str` — string slice (borrowed, immutable)
A reference to a UTF-8 sequence. String literals are `&'static str`.
```rust
let greeting: &str = "hello";
```

#### `String` — owned, heap-allocated, growable
```rust
let mut s = String::from("hello");
s.push_str(", world");  // append a &str
s.push('!');             // append a char
let replaced = s.replace("world", "Rust");

// converting between types
let s: String = "hello".to_string();
let slice: &str = &s;   // String → &str (deref coercion)
```

### Compound Types

#### Tuples
Fixed-size, can hold different types:
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;              // destructure
let first = tup.0;                // access by index
let unit: () = ();                // unit type — empty tuple
```

#### Arrays
Fixed-size, all elements same type, stored on the stack:
```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let zeros = [0; 10];              // [0, 0, 0, ..., 0] (10 elements)
let first = arr[0];
let len = arr.len();
let slice: &[i32] = &arr[1..3];  // borrow a slice
```

### Generics
Generics are placeholders for concrete types — zero-cost abstraction (monomorphized at compile time):
```rust
// generic struct
struct Wrapper<T>(T);

// generic function
fn first<T>(list: &[T]) -> &T {
    &list[0]
}

// generic impl
impl<T: std::fmt::Display> Wrapper<T> {
    fn show(&self) {
        println!("{}", self.0);
    }
}
```

#### Const Generics
Type parameters that represent compile-time constant values — useful for array sizes:
```rust
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

display_array([1, 2, 3]);       // N = 3
display_array([1, 2, 3, 4, 5]); // N = 5
```

### Heap-Allocated Box
```rust
let boxed: Box<i32> = Box::new(42);   // stores 42 on the heap
let val = *boxed;                      // dereference to get the value
```

---

## 6. Functions

### Basic Functions
```rust
fn main() {
    let result = add(5, 3);
    println!("{result}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y    // no semicolon = expression (returned implicitly)
}
```

### Statements vs Expressions
- **Statement** — performs an action, does **not** return a value (ends with `;`)
- **Expression** — evaluates to a value (no trailing `;`)

```rust
let y = {
    let x = 5;
    x * x + x       // expression — this value is assigned to y
};                   // y == 30
```

### Diverging Functions
Functions that never return use the `!` (never) type:
```rust
fn forever() -> ! {
    loop {
        // runs forever
    }
}

fn crash(msg: &str) -> ! {
    panic!("fatal: {msg}");
}
```

### Function Pointers
```rust
fn apply(f: fn(i32) -> i32, val: i32) -> i32 {
    f(val)
}

fn double(x: i32) -> i32 { x * 2 }

let result = apply(double, 5); // 10
```

---

## 7. Control Flow

### `if` / `else`
```rust
let num = 5;

if num < 0 {
    println!("negative");
} else if num > 0 {
    println!("positive");
} else {
    println!("zero");
}

// if as an expression (like a ternary)
let label = if num > 0 { "positive" } else { "non-positive" };
```

### `match`
Exhaustive pattern matching — **all** cases must be handled:
```rust
enum Coin { Penny, Nickel, Dime, Quarter }

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => 10,
        Coin::Quarter => 25,
    }
}
```

Pattern syntax:
```rust
let x = 42;
match x {
    1 | 2 | 3 => println!("small"),          // or patterns
    4..=10 => println!("medium"),          // inclusive range
    n @ 11..=100 => println!("large: {n}"),      // bind + test with @
    _ => println!("huge"),            // wildcard (catch-all)
}
```

### `if let` / `while let`
Convenient for matching a single pattern without handling all cases:
```rust
let config_max = Some(3u8);

// instead of a full match:
if let Some(max) = config_max {
    println!("max is {max}");
}

// while let — loop as long as the pattern matches
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{top}");
}
```

### `let-else` (Rust 1.65+)
Bind or diverge — great for early returns:
```rust
fn get_count(text: &str) -> u32 {
    let Some(count) = text.parse::<u32>().ok() else {
        return 0; // must diverge: return, break, panic, etc.
    };
    count
}
```

### Loops
```rust
// infinite loop
loop {
    // break to exit
    break;
}

// loop as an expression
let result = loop {
    break 42;    // the loop "returns" 42
};

// while loop
let mut n = 0;
while n < 10 {
    n += 1;
}

// for loop (iterating a range)
for i in 0..5 {          // 0, 1, 2, 3, 4
    println!("{i}");
}
for i in 0..=5 {         // 0, 1, 2, 3, 4, 5 (inclusive)
    println!("{i}");
}

// for loop with an iterator
let names = vec!["Alice", "Bob", "Charlie"];
for name in &names {     // borrows — names is still usable after
    println!("{name}");
}
```

#### Labelled Loops
Break or continue an outer loop from inside a nested loop:
```rust
'outer: for i in 0..5 {
    for j in 0..5 {
        if i + j == 6 {
            break 'outer;    // breaks the outer loop
        }
    }
}
```

---

## 8. Structs, Enums & Pattern Matching

### Structs
```rust
// named-field struct
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u8,
    active: bool,
}

// instantiation
let user = User {
    name: String::from("Alice"),
    email: String::from("alice@example.com"),
    age: 30,
    active: true,
};

// field init shorthand (when variable names match field names)
let name = String::from("Bob");
let bob = User { name, email: String::from("bob@example.com"), age: 25, active: true };

// struct update syntax (copy remaining fields from another instance)
let charlie = User { name: String::from("Charlie"), ..bob };

println!("{:?}", user);   // requires #[derive(Debug)]
```

### Tuple Structs & Unit Structs
```rust
struct Color(u8, u8, u8);          // tuple struct — accessed by index
let red = Color(255, 0, 0);
let r = red.0;

struct Marker;                      // unit struct — zero-sized, useful as type markers
```

### Methods & Associated Functions
```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

impl Rectangle {
    // associated function (constructor) — no self
    fn new(w: u32, h: u32) -> Self {
        Rectangle { width: w, height: h }
    }

    // method — takes &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method that mutates — takes &mut self
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

let mut r = Rectangle::new(10, 20);   // associated function call
println!("area: {}", r.area());        // method call
r.scale(2);
```

### Enums
```rust
#[derive(Debug)]
enum Message {
    Quit,                            // unit variant
    Echo(String),                    // tuple variant
    Move { x: i32, y: i32 },        // struct variant
    Color(u8, u8, u8),              // multi-value tuple variant
}

fn process(msg: Message) {
    match msg {
        Message::Quit             => println!("quit"),
        Message::Echo(text)       => println!("echo: {text}"),
        Message::Move { x, y }   => println!("move to ({x}, {y})"),
        Message::Color(r, g, b)  => println!("color: ({r}, {g}, {b})"),
    }
}
```

### `Option<T>`
Represents a value that may or may not exist (Rust's null alternative):
```rust
enum Option<T> {
    None,
    Some(T),
}

let some_num: Option<i32> = Some(42);
let no_num: Option<i32> = None;

// safe unwrapping
let value = some_num.unwrap_or(0);              // 42 or default
let value = some_num.unwrap_or_default();       // uses Default trait
let value = some_num.expect("should have value"); // panics with message if None

// transforming
let doubled = some_num.map(|n| n * 2);         // Some(84)
let flat = some_num.and_then(|n| if n > 0 { Some(n) } else { None });
```

### `Result<T, E>`
Represents success or failure:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

match parse_number("42") {
    Ok(n)  => println!("parsed: {n}"),
    Err(e) => println!("error: {e}"),
}
```

---

## 9. Collections

### Vec (Vector)
Growable array stored on the heap:
```rust
let mut v: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];           // vec! macro

v.push(1);
v.push(2);
v.pop();                           // removes and returns last element

let first = &v[0];                 // panics if out of bounds
let first = v.get(0);             // returns Option<&T>

// iteration
for val in &v { println!("{val}"); }          // borrow
for val in &mut v { *val += 10; }             // mutable borrow
for val in v { println!("{val}"); }           // consumes v
```

### HashMap
```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert("Alice".to_string(), 10);
scores.insert("Bob".to_string(), 20);

// access
let alice_score = scores.get("Alice");         // Option<&i32>

// insert only if key is absent
scores.entry("Charlie".to_string()).or_insert(0);

// update existing value
let counter = scores.entry("Alice".to_string()).or_insert(0);
*counter += 5;

// iterate
for (name, score) in &scores {
    println!("{name}: {score}");
}
```

### HashSet
```rust
use std::collections::HashSet;

let mut set: HashSet<i32> = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(2);                    // duplicate — ignored
println!("{}", set.len());        // 2
println!("{}", set.contains(&1)); // true
```

### Slices
A view into a contiguous sequence — does not own the data:
```rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];   // [2, 3, 4]
let full: &[i32] = &arr[..];      // entire array

let s = String::from("hello world");
let word: &str = &s[0..5];        // "hello" — string slice
```

---

## 10. Traits & Generics

### Defining & Implementing Traits
```rust
trait Summary {
    fn summarize(&self) -> String;

    // default implementation (can be overridden)
    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..20])
    }
}

struct Article { title: String, content: String }

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}
```

### Trait Bounds
```rust
// verbose syntax
fn notify<T: Summary>(item: &T) {
    println!("Breaking: {}", item.summarize());
}

// impl Trait syntax (syntactic sugar)
fn notify(item: &impl Summary) {
    println!("Breaking: {}", item.summarize());
}

// multiple bounds
fn display_and_summarize<T: Summary + std::fmt::Display>(item: &T) { /* ... */ }

// where clause (for readability)
fn complex<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: std::fmt::Debug,
{
    format!("{} {:?}", t.summarize(), u)
}
```

### Returning Traits
```rust
// return a concrete type that implements a trait (only one type allowed)
fn make_summary() -> impl Summary {
    Article { title: "News".into(), content: "Something happened".into() }
}
```

### Trait Objects (`dyn`) — Dynamic Dispatch
When you need to return or store **different** types implementing the same trait:
```rust
fn get_writer(kind: &str) -> Box<dyn Summary> {
    match kind {
        "article" => Box::new(Article { title: "Hi".into(), content: "...".into() }),
        _         => Box::new(Tweet  { username: "bot".into(), body: "...".into() }),
    }
}
```

### Common Derivable Traits
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}
```
| Trait       | Purpose                                       |
|-------------|-----------------------------------------------|
| `Debug`     | `{:?}` formatting                             |
| `Clone`     | Explicit deep copy via `.clone()`              |
| `Copy`      | Implicit bitwise copy (small stack types only) |
| `PartialEq` | `==` and `!=` comparison                      |
| `Eq`        | Marker: total equality (no NaN-like values)   |
| `Hash`      | Hashing (required for `HashMap` keys)          |
| `Default`   | Provides a default value via `Default::default()` |
| `PartialOrd`| `<`, `>`, `<=`, `>=` comparison               |
| `Ord`       | Total ordering                                |

### Associated Types
```rust
trait Iterator {
    type Item;    // associated type — set by implementor
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Supertraits
```rust
trait Printable: std::fmt::Display + std::fmt::Debug {
    fn print(&self) {
        println!("{self}");  // can use Display because it's a supertrait
    }
}
```

---

## 11. Ownership & Borrowing

### The Three Rules of Ownership
1. Each value has exactly **one owner**
2. There can only be **one owner at a time**
3. When the owner goes out of scope, the value is **dropped** (freed)

### Stack vs Heap
| Property  | Stack                   | Heap                            |
|-----------|-------------------------|---------------------------------|
| Size      | Fixed, known at compile | Dynamic, determined at runtime  |
| Speed     | Fast (push/pop)         | Slower (allocation + pointer)   |
| Access    | LIFO                    | Via pointer                     |
| Types     | `i32`, `bool`, `char`…  | `String`, `Vec`, `Box`…         |

### Move vs Copy
```rust
// Copy types (stack only) — value is duplicated
let x = 5;
let y = x;     // x is still valid (i32 implements Copy)

// Move types (heap data) — ownership transfers
let s1 = String::from("hello");
let s2 = s1;   // s1 is MOVED into s2 — s1 is no longer valid

// to keep both, clone explicitly
let s1 = String::from("hello");
let s2 = s1.clone();  // deep copy — both are valid
```

### Borrowing — References
```rust
// immutable borrow — can have many simultaneously
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{r1} {r2}");  // fine

// mutable borrow — only ONE at a time, no immutable borrows active
let mut s = String::from("hello");
let r = &mut s;
r.push_str(" world");
println!("{r}");
```

Rules:
1. You can have **either** one mutable reference **or** any number of immutable references (not both)
2. References must always be **valid** (no dangling references)

### Borrowing in Functions
```rust
fn calculate_length(s: &String) -> usize {
    s.len()   // borrows s — does not take ownership
}

fn change(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
let len = calculate_length(&s);     // immutable borrow
change(&mut s);                      // mutable borrow
```

### Partial Move
```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

let person = Person { name: String::from("Alice"), age: Box::new(30) };

// 'name' is moved out, 'age' is borrowed
let Person { name, ref age } = person;

// person.name is no longer accessible
// person.age (and age) can still be used
println!("age: {age}");
```

---

## 12. Lifetimes

Lifetimes ensure references are valid for as long as they're used. The compiler usually infers them, but sometimes you must be explicit.

### Lifetime Annotations
```rust
// this tells the compiler: the returned reference lives at least
// as long as the 'a lifetimes on x and y.
// so basically what ever values(x, y) passed to longest 
// must outlive it
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Lifetime Elision Rules
The compiler automatically infers lifetimes in common cases:
1. Each parameter with a reference gets its own lifetime
2. If there is exactly one input lifetime, it is assigned to all output lifetimes
3. If one of the parameters is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetimes

```rust
// these two signatures are equivalent — the compiler infers the lifetime:
fn first_word(s: &str) -> &str { /* ... */ }
fn first_word<'a>(s: &'a str) -> &'a str { /* ... */ }
```

### Lifetimes in Structs
A struct holding a reference **must** have a lifetime annotation:
```rust
struct Excerpt<'a> {
    text: &'a str,
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().unwrap();
let excerpt = Excerpt { text: first_sentence };
```

### `'static` Lifetime
References that live for the entire duration of the program:
```rust
let s: &'static str = "I live forever";  // string literals are 'static
```

---

## 13. Error Handling

### Unrecoverable Errors — `panic!`
```rust
panic!("something went terribly wrong");
// set RUST_BACKTRACE=1 for a full backtrace
```

### Recoverable Errors — `Result<T, E>`
```rust
use std::fs;

fn read_config() -> Result<String, std::io::Error> {
    fs::read_to_string("config.toml")
}

match read_config() {
    Ok(content)  => println!("{content}"),
    Err(e)       => eprintln!("failed to read config: {e}"),
}
```

### The `?` Operator
Propagates errors automatically — returns early with `Err` if the operation fails:
```rust
use std::fs;
use std::io;

fn read_username() -> Result<String, io::Error> {
    let mut s = fs::read_to_string("username.txt")?;  // ? returns Err early
    s.trim_end().to_string();
    Ok(s)
}
```

### Custom Error Types
```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    Unauthorized,
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(msg)  => write!(f, "not found: {msg}"),
            AppError::Unauthorized   => write!(f, "unauthorized"),
            AppError::Internal(msg)  => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}
```

### Using `thiserror` and `anyhow` (popular crates)
```rust
// thiserror — for library error types
use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("record not found: {0}")]
    NotFound(String),

    #[error("database error")]
    Database(#[from] sqlx::Error),   // auto-converts sqlx::Error → DataError
}

// anyhow — for application-level error handling
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let text = std::fs::read_to_string("config.toml")
        .context("failed to read config file")?;       // adds context to the error
    let config: Config = toml::from_str(&text)
        .context("failed to parse config")?;
    Ok(config)
}
```

---

## 14. Closures & Iterators

### Closures
Anonymous functions that can capture their environment:
```rust
let add = |a: i32, b: i32| -> i32 { a + b };
let result = add(2, 3); // 5

// types are usually inferred
let square = |x| x * x;
let nine = square(3);

// closures capture variables from their scope
let name = String::from("Rust");
let greet = || println!("Hello, {name}!");
greet();
```

Closure traits (determined by how they capture):
| Trait     | Captures by         | Can call   |
|-----------|----------------------|------------|
| `Fn`      | `&T` (immutable ref) | Many times |
| `FnMut`   | `&mut T` (mutable ref) | Many times |
| `FnOnce`  | `T` (by value/move)  | Once only  |

```rust
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}
let doubled = apply(|x| x * 2, 5); // 10

// move closure — takes ownership of captured variables
let name = String::from("Rust");
let greet = move || println!("{name}");
// name is no longer accessible here
```

### Iterators
```rust
let nums = vec![1, 2, 3, 4, 5];

// common adaptors
let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
let evens: Vec<&i32>  = nums.iter().filter(|x| *x % 2 == 0).collect();
let sum: i32           = nums.iter().sum();
let product: i32       = nums.iter().product();
let first_big           = nums.iter().find(|&&x| x > 3);       // Option<&i32>
let all_positive        = nums.iter().all(|x| *x > 0);         // bool
let any_negative        = nums.iter().any(|x| *x < 0);         // bool

// chaining
let result: Vec<i32> = (0..100)
    .filter(|x| x % 3 == 0)
    .map(|x| x * x)
    .take(5)
    .collect();

// fold (reduce)
let sum = nums.iter().fold(0, |acc, x| acc + x);

// enumerate
for (i, val) in nums.iter().enumerate() {
    println!("index {i}: {val}");
}

// zip
let names = vec!["Alice", "Bob"];
let ages  = vec![30, 25];
let people: Vec<_> = names.iter().zip(ages.iter()).collect();
// [("Alice", 30), ("Bob", 25)]
```

### Implementing Iterator for a Custom Type
```rust
struct Counter { count: u32, max: u32 }

impl Counter {
    fn new(max: u32) -> Self { Counter { count: 0, max } }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

let sum: u32 = Counter::new(5).sum(); // 1+2+3+4+5 = 15
```

---

## 15. Smart Pointers

### `Box<T>` — Heap Allocation
```rust
let b = Box::new(5);       // allocates 5 on the heap
println!("{b}");            // auto-derefs

// useful for recursive types
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### `Rc<T>` — Reference Counted (single-threaded)
Multiple owners for the same data:
```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);    // increments reference count (cheap)
let c = Rc::clone(&a);

println!("count: {}", Rc::strong_count(&a)); // 3
```

### `Arc<T>` — Atomic Reference Counted (thread-safe)
Like `Rc`, but safe to share across threads:
```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);

let handles: Vec<_> = (0..3).map(|_| {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        println!("{:?}", data);
    })
}).collect();

for h in handles { h.join().unwrap(); }
```

### `RefCell<T>` — Interior Mutability
Allows mutable borrows checked at **runtime** instead of compile-time:
```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() += 1;                 // mutable borrow at runtime
println!("{}", data.borrow());            // immutable borrow — prints 6
```

Common pattern: `Rc<RefCell<T>>` — multiple owners + interior mutability (single-threaded).

### `Cow<T>` — Clone on Write
Delays cloning until mutation is needed:
```rust
use std::borrow::Cow;

fn maybe_uppercase(s: &str, shout: bool) -> Cow<str> {
    if shout {
        Cow::Owned(s.to_uppercase())     // allocates only when needed
    } else {
        Cow::Borrowed(s)                  // zero-cost borrow
    }
}
```

---

## 16. Concurrency

### Threads
```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..5 {
        println!("spawned thread: {i}");
        thread::sleep(Duration::from_millis(100));
    }
});

// do work in main thread...

handle.join().unwrap();   // wait for the spawned thread to finish

// move data into a thread
let name = String::from("Rust");
thread::spawn(move || {
    println!("Hello from {name}");
});
```

### `Send` & `Sync` Traits
| Trait  | Meaning                                        |
|--------|------------------------------------------------|
| `Send` | Type can be **transferred** to another thread  |
| `Sync` | Type can be **referenced** from another thread |

Most types implement both. Notable exceptions: `Rc` is neither `Send` nor `Sync` (use `Arc` instead).

### Mutex (Mutual Exclusion)
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }));
}

for h in handles { h.join().unwrap(); }
println!("Result: {}", *counter.lock().unwrap()); // 10
```

### Channels (message passing)
```rust
use std::sync::mpsc;  // multi-producer, single-consumer
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("hello from thread").unwrap();
});

let msg = rx.recv().unwrap();   // blocks until a message arrives
println!("{msg}");
```

### Async / Await
For I/O-bound concurrency. Requires an async runtime (e.g., `tokio`):
```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```
```rust
use tokio;

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("{result}");
}

async fn fetch_data() -> String {
    // simulate async work
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    String::from("data loaded")
}

// running tasks concurrently
async fn parallel_work() {
    let (a, b) = tokio::join!(
        fetch_data(),
        fetch_data(),
    );
    println!("{a}, {b}");
}
```

---

## 17. Macros

### Declarative Macros (`macro_rules!`)
Pattern-based code generation:
```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

say_hello!();              // "Hello!"
say_hello!("Rust");        // "Hello, Rust!"
```

A more practical example — a custom `vec!`-like macro:
```rust
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp = Vec::new();
            $( temp.push($x); )*
            temp
        }
    };
}

let v = my_vec![1, 2, 3];
```

### Common Built-in Macros
| Macro              | Purpose                                  |
|--------------------|------------------------------------------|
| `println!`         | Print to stdout with newline             |
| `eprintln!`        | Print to stderr with newline             |
| `format!`          | Format a string (returns `String`)       |
| `vec!`             | Create a `Vec`                           |
| `todo!()`          | Mark unfinished code (panics at runtime) |
| `unimplemented!()` | Mark intentionally unimplemented code    |
| `dbg!()`           | Debug print + return value               |
| `assert!`          | Assert a condition is true               |
| `assert_eq!`       | Assert two values are equal              |
| `cfg!`             | Check a compile-time configuration       |
| `include_str!`     | Embed a file's contents as `&str`        |

### Derive Macros (Procedural)
Automatically implement traits:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
struct Config {
    name: String,
    value: i32,
}
```

### Attribute Macros
```rust
// commonly seen with frameworks like Rocket, Actix, Axum:
#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {name}!")
}
```

---

## 18. Testing

### Unit Tests
Place them in the same file, inside a `#[cfg(test)]` module:
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]                      // only compiled when running tests
mod tests {
    use super::*;                 // import from parent module

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_overflow() {
        panic!("overflow");
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err("math is broken".into())
        }
    }
}
```

### Integration Tests
Place them in the `tests/` directory at the project root:
```
my_app/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```
```rust
// tests/integration_test.rs
use my_app::add;

#[test]
fn it_adds() {
    assert_eq!(add(10, 20), 30);
}
```

### Doc Tests
Code examples in documentation comments are automatically tested:
```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_app::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Running Tests
```bash
cargo test                     # run all tests
cargo test test_add            # run tests matching a name
cargo test -- --nocapture      # show println output
cargo test -- --test-threads=1 # run tests sequentially
cargo test --doc               # run doc tests only
```

---

## 19. Unsafe Rust

### When to Use Unsafe
The `unsafe` keyword unlocks five abilities that the borrow checker cannot verify:
1. Dereference raw pointers
2. Call unsafe functions or methods
3. Access or modify mutable static variables
4. Implement unsafe traits
5. Access fields of `union` types

```rust
// raw pointers
let mut num = 5;
let r1 = &num as *const i32;     // immutable raw pointer
let r2 = &mut num as *mut i32;   // mutable raw pointer

unsafe {
    println!("r1: {}", *r1);
    *r2 = 10;
    println!("r2: {}", *r2);
}
```

### Unsafe Functions
```rust
unsafe fn dangerous() {
    // do something the compiler can't verify
}

unsafe {
    dangerous();
}
```

### Safe Abstractions over Unsafe Code
A common pattern — wrap unsafe code in a safe API:
```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

### FFI (Foreign Function Interface)
Call C functions from Rust:
```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }
}
```

Expose Rust functions to C:
```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Called from C!");
}
```

---

## 20. Useful Crates & Ecosystem

### Essential Crates
| Crate       | Purpose                           | Usage                      |
|-------------|-----------------------------------|----------------------------|
| `serde`     | Serialization / deserialization   | JSON, TOML, YAML, etc.    |
| `tokio`     | Async runtime                     | Async I/O, networking      |
| `reqwest`   | HTTP client                       | REST APIs                  |
| `clap`      | CLI argument parsing              | Command-line tools         |
| `tracing`   | Structured logging & diagnostics  | Observability              |
| `anyhow`    | Flexible error handling           | Applications               |
| `thiserror` | Derive-based error types          | Libraries                  |
| `sqlx`      | Async SQL toolkit                 | Database access            |
| `axum`      | Web framework (by tokio team)     | HTTP services              |
| `rayon`     | Data parallelism                  | Parallel iterators         |

### Web Development Stack
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
tower-http = { version = "0.5", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

### CLI Tool Stack
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
indicatif = "0.17"   # progress bars
colored = "2"         # terminal colors
```

### Finding Crates
- [crates.io](https://crates.io) — the official package registry
- [lib.rs](https://lib.rs) — curated, categorized crate discovery
- [docs.rs](https://docs.rs) — auto-generated documentation for every crate
- [blessed.rs](https://blessed.rs) — community-recommended crates by category

---

## Quick Reference Card

### Common Compiler Attributes
```rust
#[allow(dead_code)]            // suppress unused code warnings
#[allow(unused_variables)]     // suppress unused variable warnings
#[cfg(target_os = "linux")]    // conditional compilation
#[inline]                      // hint to inline a function
#[must_use]                    // warn if return value is ignored
#[deprecated]                  // mark as deprecated
```

### Formatting with `println!`
```rust
println!("{}", val);           // Display
println!("{:?}", val);         // Debug
println!("{:#?}", val);        // Pretty Debug
println!("{val}");             // inline variable (Rust 1.58+)
println!("{:.2}", 3.14159);    // 2 decimal places → "3.14"
println!("{:>10}", "right");   // right-align, width 10
println!("{:<10}", "left");    // left-align, width 10
println!("{:0>5}", 42);       // zero-pad → "00042"
```

### Useful Conversions
```rust
// String ↔ &str
let s: String = "hello".to_string();
let s: String = String::from("hello");
let r: &str = &s;
let r: &str = s.as_str();

// Numbers ↔ String
let n: i32 = "42".parse().unwrap();
let s: String = 42.to_string();

// Into / From
let s: String = "hello".into();   // uses From<&str> for String
```

---

*Built with 🦀 — contributions and corrections welcome!*
