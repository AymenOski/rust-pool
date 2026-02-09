//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

use borrow::*;

fn main() {
	let s = "hello";
	let s1 = "camelCase".to_string();
	let s2 = "olá!";

	println!("\tstr_len(\"{}\") = {}", s, str_len(s));
	println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
	println!("\tstr_len(\"{}\") = {}", s2, str_len(s2));
}


/*
* ═══════════════════════════════════════════════════════════════════════════════════
* 🌟 PRACTICAL BORROWING - HOW RUST PREVENTS MEMORY SAFETY BUGS
* ═══════════════════════════════════════════════════════════════════════════════════
* This program demonstrates how Rust's borrowing system prevents memory safety bugs
* while allowing efficient string handling. Every line teaches you something about how
* Rust keeps your code safe without sacrificing performance.
*
* 🔍 THE PROBLEM THIS SOLVES:
* In C, a function like: int str_len(const char* s) { ... }
*
* Has critical problems:
   1. The `const` keyword suggests safety, but casts can bypass it.
   2. If the string data is freed elsewhere, `s` becomes a DANGLING POINTER.
   3. If another thread modifies the string, you get DATA RACES.
   4. If you accidentally modify it, the "const" is just a suggestion.

* 📌 MEMORY MODEL:
* For `let s1 = "camelCase".to_string();`
*
* Stack:                        Heap:
* ┌─────────────┐              ┌───────────────────────┐
* │ s1 (String) │              │ 'camelCase' (9 bytes) │
* ├─────────────┤              ├───────────────────────┤
* │ ptr ────────┼─────────────→ │ (rest of allocated)   │
* │ len = 9     │              │ (capacity bytes)      │
* │ capacity=16 │              └───────────────────────┘
* └─────────────┘
*
 String Metadata (on stack, 24 bytes on 64-bit systems):
   - ptr: Pointer to heap memory (8 bytes)
   - len: Current length in bytes (8 bytes)
   - capacity: Total allocated bytes (8 bytes)

* When `s1` goes out of scope:
   &1. Rust calls drop(s1).
   &2. The heap memory (16 bytes) is deallocated.
   &3. The stack metadata goes away.

 This happens AUTOMATICALLY. No garbage collector, no manual free().
 This is Rust's RAII pattern (Resource Acquisition Is Initialization).

* ═══════════════════════════════════════════════════════════════════════════════════
*/