# Rust Fundamentals: Complete Technical Breakdown

## Table of Contents
1. [Language Philosophy](#language-philosophy)
2. [Memory Management](#memory-management)
3. [Type System](#type-system)
4. [Ownership & Borrowing](#ownership--borrowing)
5. [Lifetimes](#lifetimes)
6. [Pattern Matching](#pattern-matching)
7. [Error Handling](#error-handling)
8. [Enums & Algebraic Data Types](#enums--algebraic-data-types)
9. [Traits & Generics](#traits--generics)
10. [Concurrency](#concurrency)
11. [Macro System](#macro-system)
12. [Performance Characteristics](#performance-characteristics)
13. [Unsafe Rust](#unsafe-rust)
14. [Standard Library Essentials](#standard-library-essentials)

---

## Language Philosophy

**The Three Pillars: Safety, Speed, Concurrency**

Rust fundamentally solves the problem that has plagued systems programming for decades: **how to prevent memory safety bugs while maintaining zero-cost abstractions and performance equivalent to C/C++**.

- **Memory Safety Without Garbage Collection**: Uses compile-time borrow checking instead of runtime GC, eliminating pause times
- **No Null Pointer Exceptions**: The type system makes invalid states unrepresentable; `None` is explicit via `Option<T>`
- **Data Race Prevention**: The compiler prevents data races at compile-time through exclusive ownership rules
- **Fearless Concurrency**: You can't create race conditions that compile; concurrency primitives are composable
- **Zero-Cost Abstractions**: High-level constructs compile to the same assembly as hand-written C would produce
- **Ownership-Based Resource Management**: RAII (Resource Acquisition Is Initialization) is automatic via drop semantics

Rust achieves this through an unprecedented type system that makes the compiler incredibly strict about aliasing and mutation.

---

## Memory Management

### Stack vs Heap

**Stack Allocation** (Default for most types):
- Fixed-size data allocated at compile time
- LIFO (Last In First Out) allocation/deallocation
- Extremely fast (single pointer increment on x86-64)
- Automatic cleanup when scope exits
- Cache-friendly (linear memory access patterns)

**Heap Allocation** (For dynamic-sized data):
- Variable-size data, size determined at runtime
- Accessed via pointer stored on stack
- Slightly slower due to pointer indirection
- Manual allocation/deallocation in C/C++; automatic in Rust
- Can be fragmented, less cache-friendly

**Example of stack vs heap**:
```rust
let x = 5;                    // 5 lives on stack, dies when x goes out of scope
let s = String::from("hi");   // "hi" stored on heap, pointer/cap/len on stack
```

### Drop Trait & RAII

Every value has a `Drop` implementation (often implicit). When a value goes out of scope, `drop()` is called:

```rust
struct File {
    // implementation
}

impl Drop for File {
    fn drop(&mut self) {
        // Close file descriptor, deallocate resources
    }
}

fn main() {
    let f = File::open("data.txt");
    // ... use file ...
} // f drops here automatically, file is closed
```

This is compile-time guaranteed. No destructor forgetting, no leaks in normal code.

### Memory Layout

Rust gives you explicit control over memory layout:

```rust
#[repr(C)]  // C-compatible layout for FFI
struct Point {
    x: f64,  // offset 0, 8 bytes
    y: f64,  // offset 8, 8 bytes
}

#[repr(transparent)]  // Guaranteed same layout as wrapped type
struct UserId(u64);

#[repr(packed)]  // No padding
struct Packed {
    a: u8,  // offset 0
    b: u32, // offset 1 (no padding in packed)
}

// Default Rust layout may reorder fields for optimal packing:
struct Optimized {
    a: u8,   // 1 byte
    b: u32,  // 4 bytes (may come before 'a' to minimize padding)
    c: u16,  // 2 bytes
}
```

---

## Type System

### Primitive Types

**Integers**: `i8, i16, i32, i64, i128, isize` (signed) and `u8, u16, u32, u64, u128, usize` (unsigned)
- `isize`/`usize` platform-dependent (32 on 32-bit, 64 on 64-bit)
- Overflow behavior: panics in debug, wraps in release (changeable with `overflowing_*` methods)
- Literals: `0xFF` (hex), `0b1010` (binary), `0o77` (octal), `1_000_000` (underscores for readability)

**Floats**: `f32, f64` (IEEE 754)
- No total ordering (NaN != NaN)
- Not hashable or usable in some standard collections
- Literal parsing: `1.0`, `1e10`, `f32::INFINITY`, `f32::NAN`

**Booleans**: `bool` (not castable to int)

**Characters**: `char` (Unicode scalar value, 4 bytes, not ASCII)

**Never Type**: `!` (function never returns)
```rust
fn diverge() -> ! {
    panic!("This function never returns");
}
```

### Compound Types

**Tuples** (heterogeneous, fixed-size):
```rust
let tup: (i32, f64, bool) = (500, 6.4, true);
let (x, y, z) = tup;  // destructuring
let fifth = tup.0;     // field access by index
```

**Arrays** (homogeneous, fixed-size, stack-allocated):
```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let same = [3; 5];  // [3, 3, 3, 3, 3]
let elem = arr[0];  // bounds-checked, panics if out of bounds
```

**Slices** (dynamically-sized view into array/Vec):
```rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..3];  // [2, 3], exclusive end
let slice = &arr[..];             // entire array
let slice = &arr[2..];            // from 2 to end
```

**Strings** vs **String Slices**:
```rust
let s: &str = "hello";           // string literal, immutable, static lifetime
let s: String = String::from("hello");  // heap-allocated, mutable, owned
let s: &str = &s[..];            // create string slice from String
let s: &[u8] = b"bytes";         // byte string
```

---

## Ownership & Borrowing

### The Ownership Model (Core Concept)

Every value has exactly **one owner**. Three rules:
1. Each value has one owner
2. You can borrow the value from the owner (immutable or mutable)
3. When the owner goes out of scope, the value is dropped

```rust
fn main() {
    let s1 = String::from("hello");  // s1 owns string
    let s2 = s1;                     // s1's ownership moves to s2, s1 is invalid
    // println!("{}", s1);           // ERROR: s1 no longer owns the string
    println!("{}", s2);              // OK
}
```

**Move Semantics**: For non-`Copy` types, assignment transfers ownership (bitwise copy of stack representation, not deep copy):
```rust
let s1 = String::from("hello");
let s2 = s1;  // moves
// s1 is now uninitialized, can't be used
```

**Copy Trait**: Small types (integers, floats, booleans, chars) implement `Copy`, so assignment copies semantics:
```rust
let x = 5;
let y = x;  // x is still valid, Copy makes implicit copy
```

### Borrowing (Immutable & Mutable)

**Immutable Borrow** (`&T`):
```rust
fn len(s: &String) -> usize {
    s.len()
}

let s = String::from("hello");
let length = len(&s);  // borrows immutably
println!("{}", s);     // still valid, can use elsewhere
```

Rules:
- Multiple immutable borrows allowed simultaneously
- Can't mutate through immutable borrow
- Borrowed value can't be moved while borrow exists

**Mutable Borrow** (`&mut T`):
```rust
fn append(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

let mut s = String::from("hello");
append(&mut s, " world");
println!("{}", s);  // "hello world"
```

Rules:
- **At most one mutable borrow at a time** (exclusive access)
- Can't have immutable borrow while mutable borrow exists
- Can't have mutable borrow while immutable borrow exists
- This prevents data races at compile-time

**Borrow Checker Visualization**:
```rust
let mut s = String::from("hello");
let r1 = &s;      // immutable borrow starts
let r2 = &s;      // another immutable borrow
println!("{} {}", r1, r2);  // r1, r2 last used here
let r3 = &mut s;  // mutable borrow starts (allowed, no active immutable borrows)
println!("{}", r3);
```

### Interior Mutability

For cases where you need mutation behind immutable references:

**`Cell<T>`** (single-threaded, not for references):
```rust
use std::cell::Cell;

let x = Cell::new(5);
x.set(6);
let value = x.get();
```

**`RefCell<T>`** (single-threaded, allows mutable access through `borrow_mut()`):
```rust
use std::cell::RefCell;

let s = RefCell::new(String::from("hello"));
s.borrow_mut().push_str(" world");
println!("{}", s.borrow());  // "hello world"
// Panics at runtime if you borrow_mut() while already borrowed
```

**`Mutex<T>`** and **`RwLock<T>`** (thread-safe interior mutability):
```rust
use std::sync::Mutex;

let s = Mutex::new(String::from("hello"));
s.lock().unwrap().push_str(" world");  // blocks other threads
```

---

## Lifetimes

Lifetimes are annotations that tell the compiler how long references are valid. They solve the dangling reference problem.

### Basic Lifetime Syntax

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

let s1 = String::from("long");
let s2 = String::from("short");
let result = longest(s1.as_str(), s2.as_str());
// result is valid as long as both s1 and s2 are valid
```

The `'a` lifetime parameter means: "The returned reference is valid for as long as both input references are valid."

### Lifetime Rules (Elision)

Compiler automatically infers lifetimes in these cases:
1. Each parameter gets its own lifetime
2. If one input parameter, output gets that lifetime
3. If `&self` or `&mut self` method, output gets `self`'s lifetime

```rust
fn first_word(s: &str) -> &str {  // implicitly: fn first_word<'a>(s: &'a str) -> &'a str
    &s[..s.find(' ').unwrap_or(s.len())]
}
```

### Struct Lifetimes

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,  // holds reference to string slice
}

fn main() {
    let s = String::from("important words");
    let excerpt = ImportantExcerpt { part: &s };
    // excerpt can't outlive s
}
```

### Static Lifetime

```rust
let s: &'static str = "hello";  // string literals live for entire program
```

### Lifetime Bounds

```rust
fn longest_with_announcement<'a, T: std::fmt::Display>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

---

## Pattern Matching

Rust's pattern matching is exhaustive—the compiler ensures all cases are handled.

### Basic Patterns

```rust
match value {
    1 => println!("one"),
    2 => println!("two"),
    3..=5 => println!("three to five"),
    _ => println!("anything else"),  // catch-all
}
```

### Destructuring

**Structs**:
```rust
struct Point { x: i32, y: i32 }

let p = Point { x: 0, y: 7 };
match p {
    Point { x, y } => println!("{}, {}", x, y),
}

// Partial destructuring:
match p {
    Point { x: 0, .. } => println!("x is 0"),  // .. ignores remaining fields
    _ => (),
}
```

**Tuples**:
```rust
let (x, y, z) = (1, 2, 3);
let (a, .., z) = (1, 2, 3, 4, 5);  // a=1, z=5, middle ignored
```

**Enums**:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

match msg {
    Message::Quit => (),
    Message::Move { x, y } => println!("move to {}, {}", x, y),
    Message::Write(s) => println!("{}", s),
    Message::ChangeColor(r, g, b) => (),
}
```

### Guard Clauses

```rust
match x {
    1..=5 if x % 2 == 0 => println!("even in 1-5"),
    _ => println!("something else"),
}
```

### Binding in Patterns

```rust
match point {
    Point { x: 0, y } => println!("x is 0, y is {}", y),
    Point { x, y: 0 } => println!("y is 0, x is {}", x),
    Point { ref x, y } => {  // ref borrows
        println!("borrowed x: {}", x);
    },
}
```

### if let (Shorthand)

```rust
if let Some(x) = maybe_value {
    println!("{}", x);
} else {
    println!("no value");
}

// while let for looping
while let Some(x) = iter.next() {
    println!("{}", x);
}
```

---

## Error Handling

Rust has no exceptions. Errors are values.

### Result<T, E>

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}
```

**Propagation with `?` operator**:
```rust
fn read_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;  // short-circuits on error
    Ok(content)
}
```

The `?` operator converts error types via `From` trait.

**Methods on Result**:
```rust
result.unwrap();                    // returns Ok(T) or panics on Err
result.unwrap_or(default);          // returns Ok(T) or default on Err
result.expect("message");           // unwrap with custom panic message
result.map(|x| x + 1);              // transforms Ok value
result.and_then(|x| divide(x, 2));  // chains operations
result.or_else(|_| Ok(default));    // use different value on error
```

### Option<T>

```rust
enum Option<T> {
    Some(T),
    None,
}

fn find(v: &[i32], target: i32) -> Option<usize> {
    for (i, &x) in v.iter().enumerate() {
        if x == target { return Some(i); }
    }
    None
}

match find(&[1, 2, 3], 2) {
    Some(i) => println!("found at {}", i),
    None => println!("not found"),
}

// Using methods:
find(&[1, 2, 3], 2)
    .map(|i| i + 1)
    .unwrap_or(0)
```

### Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
enum ParseError {
    Empty,
    InvalidFormat,
    Overflow,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty input"),
            ParseError::InvalidFormat => write!(f, "invalid format"),
            ParseError::Overflow => write!(f, "number too large"),
        }
    }
}

impl std::error::Error for ParseError {}
```

### Panic

Unrecoverable errors:
```rust
panic!("something went very wrong");  // stops execution, unwinds stack

// panic in release mode
assert!(x > 0, "x must be positive");  // debug_assert only in debug
```

---

## Enums & Algebraic Data Types

### Enums with Data

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

**Associated functions**:
```rust
impl IpAddr {
    fn new_v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
        IpAddr::V4(a, b, c, d)
    }
}

let home = IpAddr::new_v4(127, 0, 0, 1);
```

### Discriminant and Size

```rust
enum Discriminant {
    Unit,           // discriminant = 0
    Newtype(u32),   // discriminant = 1
    Struct { x: u32 },  // discriminant = 2
    Tuple(u32, u64),    // discriminant = 3
}

// Size = largest variant + discriminant storage
std::mem::size_of::<Discriminant>();  // typically 16 bytes (u64 field + discriminant)

// Niche optimization:
let opt: Option<NonZeroU32> = None;  // uses 0 as discriminant (same size as u32!)
std::mem::size_of::<Option<NonZeroU32>>();  // 4 bytes, no overhead
```

### Zero-Cost Abstractions

```rust
enum Bool {
    True,
    False,
}

// Compiles to same assembly as plain bool
fn from_bool(b: Bool) -> i32 {
    match b {
        Bool::True => 1,
        Bool::False => 0,
    }
}
```

---

## Traits & Generics

### Trait Definition

```rust
trait Animal {
    fn speak(&self) -> String;
    
    fn describe(&self) -> String {  // default implementation
        format!("An animal: {}", self.speak())
    }
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) -> String {
        String::from("Woof!")
    }
}

let dog = Dog;
println!("{}", dog.speak());
```

### Trait Bounds

```rust
fn print_it<T: std::fmt::Display>(x: T) {
    println!("{}", x);
}

fn notify<T: Animal>(animal: T) {
    println!("{}", animal.speak());
}

// Multiple bounds:
fn process<T: Clone + std::fmt::Debug>(x: T) {
    println!("{:?}", x.clone());
}

// Where clause (clearer with many bounds):
fn process<T>(x: T)
where
    T: Clone + std::fmt::Debug,
{
    println!("{:?}", x.clone());
}
```

### Trait Objects

```rust
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

for animal in animals {
    println!("{}", animal.speak());
}
```

**Static dispatch vs Dynamic dispatch**:
- Generics: monomorphization, each type specialized at compile-time, zero overhead
- Trait objects (`dyn`): one code path, vtable lookup at runtime, small overhead

### Inherent Impl

```rust
impl String {
    fn shout(&self) -> String {
        format!("{}!", self.to_uppercase())
    }
}

// Methods that don't require trait impl
let s = String::from("hello");
println!("{}", s.shout());
```

### Associated Types

```rust
trait Iterator {
    type Item;  // associated type, not a generic parameter
    
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {
        // ...
    }
}
```

### Important Standard Traits

**`Copy`**: Marker trait, automatic bitwise copy
```rust
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }
```

**`Clone`**: Explicit deep copy
```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // explicit, s1 still valid
```

**`Default`**: Default value construction
```rust
impl Default for MyType {
    fn default() -> Self {
        MyType { /* defaults */ }
    }
}

let x = MyType::default();
let vec: Vec<i32> = vec![0; 10];  // uses Default::default for i32
```

**`Debug`**: Debuggable output (`{:?}`)
```rust
#[derive(Debug)]
struct Point { x: i32 }
println!("{:?}", Point { x: 5 });  // "Point { x: 5 }"
```

**`Display`**: Human-readable output (`{}`)
```rust
impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

**`PartialEq`, `Eq`**: Equality comparison
```rust
#[derive(PartialEq)]
struct Point { x: i32, y: i32 }

assert_eq!(Point { x: 1, y: 2 }, Point { x: 1, y: 2 });
```

**`PartialOrd`, `Ord`**: Ordering
```rust
#[derive(PartialOrd, Ord)]
struct Point { x: i32, y: i32 }
```

**`Hash`**: Hash function for use in HashMaps/Sets
```rust
#[derive(Hash)]
struct Point { x: i32, y: i32 }

use std::collections::HashMap;
let mut map = HashMap::new();
map.insert(Point { x: 1, y: 2 }, "location");
```

**`Into<T>`, `From<T>`**: Conversions
```rust
impl From<&str> for String {
    fn from(s: &str) -> String {
        String::from(s)  // or s.to_string()
    }
}

// Into automatically implemented if From implemented
let s: String = "hello".into();
```

---

## Concurrency

### Threads

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi from spawned thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

for i in 1..5 {
    println!("hi from main thread: {}", i);
    thread::sleep(Duration::from_millis(1));
}

handle.join().unwrap();  // wait for thread to finish
```

**Move Closure** (to move ownership into thread):
```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", v);  // v moved into closure
});
handle.join().unwrap();
// v is invalid here
```

### Channels (MPSC)

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(42).unwrap();
});

let received = rx.recv().unwrap();
println!("got {}", received);

// Multiple senders:
let (tx, rx) = mpsc::channel();
let tx2 = tx.clone();
```

### Mutex

```rust
use std::sync::Mutex;

let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 6;
}
println!("{:?}", m);  // Mutex { data: 6 }

// With Arc for shared ownership:
use std::sync::Arc;
let m = Arc::new(Mutex::new(0));
let m2 = Arc::clone(&m);

thread::spawn(move || {
    *m2.lock().unwrap() += 1;
});
```

### RwLock

```rust
use std::sync::RwLock;

let data = RwLock::new(vec![1, 2, 3]);

// Multiple readers allowed:
let r1 = data.read().unwrap();
let r2 = data.read().unwrap();

// Exclusive writer:
let mut w = data.write().unwrap();
w.push(4);
```

### Sync and Send Traits

- **`Send`**: Can be sent between threads (ownership transferred)
- **`Sync`**: Can be shared between threads safely (`&T` can be sent)

```rust
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

assert_send::<String>();   // String is Send
assert_sync::<String>();   // String is Sync
assert_send::<Rc<i32>>();  // ERROR: Rc is not Send
assert_sync::<Cell<i32>>(); // ERROR: Cell is not Sync
```

Compiler automatically implements these for types that don't contain non-Send/Sync types.

### Async/Await (Tokio)

```rust
use tokio::task;

#[tokio::main]
async fn main() {
    let task = task::spawn(async {
        println!("in task");
    });
    
    task.await.unwrap();
}
```

Async is zero-cost abstraction; compiled to state machine.

---

## Macro System

### Declarative Macros (macro_rules!)

```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    };
}

let v = vec![1, 2, 3];
```

**Matchers**:
- `expr`: expression
- `stmt`: statement
- `tt`: token tree
- `item`: item (function, struct, etc.)
- `path`: path (e.g., `std::vec::Vec`)

**Repetition**:
- `$(...)*`: zero or more
- `$(...)+`: one or more
- `$(...),*`: comma-separated list

### Procedural Macros

Macros that operate on token streams (advanced, requires separate crate):

```rust
// In a proc-macro crate:
#[proc_macro_derive(Debug)]
pub fn derive_debug(input: TokenStream) -> TokenStream {
    // Parse input, generate code
}
```

Used for derive macros, custom attributes, function-like macros.

### Common Macros

```rust
println!("{}", x);              // formatted output
eprintln!("{}", x);             // formatted error output
panic!("message");              // unrecoverable error
assert!(condition);             // panic if false
debug_assert!(condition);       // only in debug
todo!();                        // unfinished code, panics if reached
unreachable!();                 // marks unreachable code
```

---

## Performance Characteristics

### Zero-Cost Abstractions

**Generics**:
```rust
fn generic<T>(x: T) { }  // monomorphized at compile-time
```

Compiles to separate code for each type, no runtime overhead.

**Trait Objects**:
```rust
fn trait_obj(obj: &dyn Trait) { }  // uses vtable, tiny runtime cost
```

Small vtable lookup overhead, but single code path.

**Closures**:
```rust
let add = |x, y| x + y;  // zero-cost closure, inlined
```

For non-capturing closures, compiles to function pointer. For capturing, compiles to struct with field access.

### Inlining

```rust
#[inline]       // hint to inline
#[inline(always)]  // force inline
#[inline(never)]   // never inline

fn small_function() { }  // likely inlined automatically
```

### Optimization Levels

- `opt-level = 0`: no optimization (default debug)
- `opt-level = 2`: moderate optimization (default release)
- `opt-level = 3`: aggressive optimization
- `opt-level = "z"`: optimize for size

### Memory Efficiency

**String Optimization**:
```rust
let s = String::from("hello");
// Allocation: capacity > len to avoid reallocation
println!("{} {}", s.len(), s.capacity());  // "5 5" or more
```

**Vec Optimization**:
```rust
let mut v = Vec::with_capacity(100);  // pre-allocate
v.push(1);  // no reallocation
```

**Enum Niche Optimization**:
```rust
// These are 8 bytes, not 16:
Option<NonZeroU64>
Option<&T>
Option<Box<T>>
```

---

## Unsafe Rust

Unsafe code opts out of borrow checker; programmer takes responsibility for safety.

### Unsafe Operations

**Dereferencing raw pointers**:
```rust
let x = 5;
let r = &x as *const i32;  // raw pointer
unsafe {
    println!("{}", *r);  // dereference
}
```

**Calling unsafe functions**:
```rust
extern "C" {
    fn my_c_function();
}

unsafe {
    my_c_function();  // could do anything, undefined behavior if unsafe
}
```

**Mutating global state**:
```rust
static mut COUNTER: u32 = 0;

unsafe {
    COUNTER += 1;
}
```

**Accessing union fields**:
```rust
union MyUnion {
    f1: u32,
    f2: f32,
}

unsafe {
    let u = MyUnion { f1: 1 };
    println!("{}", u.f2);  // might be invalid
}
```

### Safe Abstractions Over Unsafe

```rust
pub fn safe_function(x: i32) -> i32 {
    unsafe {
        // UNSAFE CODE HERE
        // But we guarantee no undefined behavior escapes
    }
}
```

Safe public API, unsafe implementation allowed if invariants maintained.

---

## Standard Library Essentials

### Collections

**Vec<T>** (dynamic array):
```rust
let mut v = Vec::new();
v.push(1);
v.pop();
v[0];
v.len();
v.iter();  // iterator (borrowed)
v.into_iter();  // iterator (owned, consumes v)
```

**HashMap<K, V>**:
```rust
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");
map.get("key");  // returns Option
```

**HashSet<T>**:
```rust
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert(1);
set.contains(&1);
```

**BTreeMap/BTreeSet**: Ordered by key (slower insert, faster iteration/range queries)

**VecDeque<T>**: Double-ended queue

**LinkedList<T>**: Doubly-linked list (generally avoid, Vec is usually better)

### Iterators

```rust
let v = vec![1, 2, 3];

// Iteration:
for x in v.iter() { println!("{}", x); }
for x in &v { println!("{}", x); }  // same as above

// Methods:
v.iter().map(|x| x * 2).filter(|x| x > 2).sum::<i32>();

// into_iter (consumes v):
for x in v { println!("{}", x); }  // v is invalid after

// Custom iterator:
struct Counter {
    count: u32,
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.count += 1;
        if self.count < 6 { Some(self.count) } else { None }
    }
}
```

### File I/O

```rust
use std::fs;
use std::io::{self, Read};

// Read to string:
let contents = fs::read_to_string("file.txt")?;

// Read lines:
use std::io::BufRead;
let file = fs::File::open("file.txt")?;
let reader = io::BufReader::new(file);
for line in reader.lines() {
    println!("{}", line?);
}

// Write:
fs::write("file.txt", "content")?;
```

### Path and PathBuf

```rust
use std::path::{Path, PathBuf};

let path = Path::new("file.txt");
let pathbuf = PathBuf::from("file.txt");

path.exists();
path.is_file();
path.parent();
path.extension();
```

### Formatting

```rust
println!("{}", x);              // Display
println!("{:?}", x);            // Debug
println!("{:#?}", x);           // Pretty Debug
println!("{:x}", x);            // Hex
println!("{:b}", x);            // Binary
println!("{:.2}", 3.14159);     // 2 decimals
println!("{:0width$}", x, width=5);  // Padding
```

### Crates to Know

- `serde`: Serialization/deserialization
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `sqlx`: SQL query builder
- `clap`: Command-line parsing
- `regex`: Regular expressions
- `log`: Logging
- `thiserror`: Error type derive macros
- `anyhow`: Flexible error handling

---

## Advanced Type System Features

### Associated Constants

```rust
trait Config {
    const MAX_SIZE: usize;
}

impl Config for MyType {
    const MAX_SIZE: usize = 1024;
}
```

### HRTB (Higher-Ranked Trait Bounds)

```rust
// For all lifetimes 'a:
fn call<F: for<'a> Fn(&'a i32) -> &'a i32>(f: F) {
    // ...
}
```

### Variance

- **Covariance**: `T` can be substituted with subtype
- **Contravariance**: `T` can be substituted with supertype
- **Invariance**: no substitution

```rust
fn foo(x: &'long T) { }
fn foo(x: &'short T) { }  // covariant in lifetime: 'long > 'short

fn foo(x: fn(&'long T)) { }
fn foo(x: fn(&'short T)) { }  // contravariant in lifetime param
```

### Phantom Types

```rust
use std::marker::PhantomData;

struct PhantomZeroSized<T> {
    data: PhantomData<T>,  // zero-size, but compiler knows about T
}
```

---

## Module System

```rust
// File structure determines modules:
// src/
//   main.rs
//   lib.rs
//   module.rs

// In lib.rs or main.rs:
mod module {
    pub fn public_fn() { }
    fn private_fn() { }
}

mod nested {
    pub mod inner {
        pub fn fn() { }
    }
}

// In another file (module.rs):
pub fn public_fn() { }

// Usage:
module::public_fn();
nested::inner::fn();

use module::public_fn;  // import
pub use module::public_fn;  // re-export
```

---

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("expected");
    }
    
    #[test]
    #[ignore]
    fn expensive_test() {
        // runs only with: cargo test -- --include-ignored
    }
}
```

---

## FFI (Foreign Function Interface)

```rust
extern "C" {
    fn c_function(x: i32) -> i32;
}

fn rust_function() {
    unsafe {
        c_function(42);
    }
}

// Export to C:
#[no_mangle]
pub extern "C" fn rust_function() -> i32 {
    42
}
```

---

## Commonly Misunderstood Concepts

### Why Rust is Hard

1. **Borrow Checker**: Most difficulty stems from learning to think in terms of ownership
2. **Lifetime Elision**: Implicit lifetimes make manual annotations confusing
3. **Trait Coherence**: Can't implement external traits for external types
4. **Turbofish**: `::<>` syntax for explicit type parameters
```rust
let nums: Vec<_> = (0..10).map(|x| x.to_string()).collect();
let nums: Vec<String> = (0..10).map(|x| x.to_string()).collect();
let nums = (0..10).map(|x| x.to_string()).collect::<Vec<String>>();  // turbofish
```

### Autoref/Autoderef

Methods are called on type, calling deref and taking &self as needed:
```rust
let s = String::from("hello");
s.len();  // automatically: (&s).len()
```

### Type Inference

```rust
let x = 5;  // inferred as i32 (default integer type)
let x: i64 = 5;  // explicitly i64

// With generics:
let v: Vec<i32> = vec![];
let v = Vec::<i32>::new();
```

### Sized vs Unsized Types

- **Sized** (`T`): Size known at compile-time (default)
- **Unsized** (`[T]`, `str`, `dyn Trait`): Size unknown, usually need reference

```rust
fn foo<T: ?Sized>(x: &T) { }  // ? Sized = may be Sized or Unsized
```

---

## Common Patterns

### Builder Pattern

```rust
struct Config {
    host: String,
    port: u16,
}

struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
}

impl ConfigBuilder {
    fn new() -> Self { ConfigBuilder { host: None, port: None } }
    
    fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }
    
    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.ok_or("missing host")?,
            port: self.port.ok_or("missing port")?,
        })
    }
}

let config = ConfigBuilder::new()
    .host("localhost")
    .port(8080)
    .build()?;
```

### RAII

```rust
struct File {
    fd: i32,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd); }
    }
}

fn main() {
    let f = File { fd: open_file() };
    // use f...
}  // f.drop() called automatically
```

### Type-State Pattern

```rust
struct Closed;
struct Open;

struct File<State = Closed> {
    fd: i32,
    _state: PhantomData<State>,
}

impl File<Closed> {
    fn open(fd: i32) -> File<Open> {
        File { fd, _state: PhantomData }
    }
}

impl File<Open> {
    fn read(&self) -> String { /* ... */ }
    
    fn close(self) -> File<Closed> {
        File { fd: self.fd, _state: PhantomData }
    }
}

let f = File { fd: -1, _state: PhantomData };
let f = f.open(3);
let _ = f.read();  // OK
let f = f.close();
let _ = f.read();  // COMPILE ERROR
```

---

## Compilation and Optimization

### Build Process

1. Parsing: Convert source to AST
2. Expansion: Expand macros
3. Type Checking: Type inference, borrow checking
4. Codegen: LLVM IR generation
5. Linking: Link object files

### LTO (Link-Time Optimization)

```toml
[profile.release]
lto = true  # slower compile, better runtime
```

### Stripping Symbols

```toml
[profile.release]
strip = true  # smaller binary
```

### Codegen Units

```toml
[profile.release]
codegen-units = 1  # slower compile, better optimization (benefits of LTO without LTO)
```

---

## Performance Tips

1. **Profile before optimizing** (`cargo flamegraph`, `perf`)
2. **Use `&str` instead of `&String`** in function parameters
3. **Avoid cloning**: Use references or `Cow<T>`
4. **Use iterators** instead of loops (more optimizable)
5. **Enable LTO and `codegen-units = 1`** for releases
6. **Use SIMD** for performance-critical code
7. **Inline hot functions** with `#[inline]`
8. **Pre-allocate collections** with `with_capacity()`

---

## Comparison to Other Languages

**vs C++**:
- No null pointers
- Borrow checker prevents entire classes of bugs
- No undefined behavior in safe code
- Faster compilation than large C++ projects
- Runtime similar or identical

**vs Go**:
- No garbage collection overhead
- More control over memory layout
- Stronger type system
- Less "magical", more explicit

**vs Python**:
- Compiled, much faster
- Requires thinking about types and ownership
- No dynamic typing

**vs Java**:
- No garbage collection pauses
- More control over memory
- Stronger type system (no null)

---

## Learning Progression

**Beginner** (1-2 weeks):
- Variables, ownership, references
- If/match/loops
- Functions
- Structs and basic impl

**Intermediate** (2-4 weeks):
- Lifetimes
- Traits
- Error handling (Result/Option)
- Collections (Vec, HashMap)
- Pattern matching
- Module system

**Advanced** (1-3 months):
- Async/await
- Procedural macros
- Unsafe code
- Advanced trait bounds
- Type system features
- Performance profiling

**Expert** (ongoing):
- LLVM understanding
- Compiler internals
- Advanced macro metaprogramming
- Custom derive macros
- FFI best practices

