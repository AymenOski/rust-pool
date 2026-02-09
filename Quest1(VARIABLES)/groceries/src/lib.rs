//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file

pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

// * clone() creates a copy of the String, which is necessary because we are passing a reference to a String.
// * reference in memory means that we are passing a pointer to the original String, so if we were to push it directly, we would be pushing the reference itself, which is not what we want. We want to push a new String that has the same content as the original, hence the need for clone().
// * but we know in c a pointer means that we can edit that value, why can't we edit in here also? Because in Rust, when we have a reference to a String, we are not allowed to modify the original String through that reference. This is part of Rust's ownership and borrowing rules, which ensure memory safety. If we were allowed to modify the original String through a reference, it could lead to data races and other unsafe behavior like dangling pointers; so only the owner of the String can modify it, and references are read-only by default. To modify the original String, we would need to have a mutable reference, but in this case, we are just trying to push a new String into the vector, so we need to create a new String that has the same content as the original, which is why we use clone() to create that new String.
// pub fn insert(vec: &mut Vec<String>, val: &String) {
//     vec.push(val.clone());
// }

//* to_string() is a method from the ToString trait, which for &str allocates a new String.
//* under the hood, it just calls to_owned() for &str, so it's essentially the same as using to_owned() in this context.
// pub fn insert(vec: &mut Vec<String>, val: &str) {
//     vec.push(val.to_string());
// }

//* into() is from the Into trait, and since &str implements Into<String>, it does the conversion.
// pub fn insert(vec: &mut Vec<String>, val: &str) {
//     vec.push(val.into());
// }


//* to_owned() is from the ToOwned trait, which creates an owned version of the borrowed type.
// pub fn insert(vec: &mut Vec<String>, val: &str) {
//     vec.push(val.to_owned());
// }    
    

// * ═══════════════════════════════════════════════════════════════════════════════════

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}


//~ pub fn at_index(slice: &[String], index: i32) -> &str {
//~     &slice[index as usize]
//~ }

/*
* ═══════════════════════════════════════════════════════════════════════════════════
* 🧠 WHY DOES RUST FORCE USIZE ON INDEXING ARRAYS?
* ═══════════════════════════════════════════════════════════════════════════════════
 Indexing with a number bigger than what the platform can address (e.g., 64-bit or 32-bit) is unsafe.
 Indexing with a negative number:
 → Interpreted as a huge positive number → reads random memory → very dangerous.

 If you want to index into an array, slice, or Vec:
 - You are only allowed to use usize.
   → Guarantees it can never be negative.
   → Guarantees it can hold any valid memory address on this platform.
* ═══════════════════════════════════════════════════════════════════════════════════
*/

/*
    * &[String] 
    - this is a slice of Strings, which is a view into a contiguous sequence of Strings in memory.
    - String → each element inside is an owned String (heap-allocated string)
        → why its heap-allocated? because String is a growable, heap-allocated data structure that can store a variable amount of text. It consists of three main components: a pointer to the heap memory where the string data is stored, the length of the string, and the capacity of the allocated memory. When you create a String, it allocates memory on the heap to store the string data, allowing it to grow as needed. This is in contrast to &str, which is a string slice that references a string stored elsewhere (often in read-only memory) and does not have ownership or the ability to grow.
        → the heap is a region of memory used for dynamic allocation, and why do we have heap-allocated strings? because they can grow and shrink in size, which is necessary for many applications where the amount of text is not known at compile time.
*/