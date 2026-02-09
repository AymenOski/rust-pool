pub fn str_len(s: &str) -> usize {
    s.chars().count()
}

/*
* WHY THIS DESIGN?
 String literals like "hello" are compile-time constants stored in read-only memory.
 Making them &str (immutable borrowed slices) ensures:
   - Nobody can accidentally modify the binary at runtime.
   - We don't waste heap memory on unchanging data.
   - Multiple parts of your program can safely share the same literal.



* 📌 BORROWING vs OWNERSHIP:
* When you call `str_len(s)` where `s: &str`, Rust BORROWS the slice.
 The function cannot:
   - Modify the string (immutable borrow).
   - Take ownership (it's temporary).
   - Invalidate the original data while reading.

* This is Rust's answer to C's safety problem:
 "How do I safely pass data to a function without risk of use-after-free?"
 Answer: BORROW it, don't transfer ownership.
* ═══════════════════════════════════════════════════════════════════════════════════
*/