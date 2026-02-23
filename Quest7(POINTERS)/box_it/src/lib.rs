//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut result = Vec::new();

    // Split by spaces
    for part in s.split_whitespace() {
        let val: u32;
        
        if part.ends_with('k') {
            let num_str = &part[..part.len() - 1];
            let num: f64 = num_str.parse().unwrap_or(0.0);
            val = (num * 1000.0) as u32;
        } else {
            // Normal number
            val = part.parse().unwrap_or(0);
        }

        result.push(Box::new(val));
    }
    
    result
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|boxed| *boxed).collect()
}

/*
    * Q & A: 
    * Q1: What are smart pointers in Rust?
    - A1: Smart pointers in Rust are data structures that not only act like pointers but also have additional metadata and capabilities. They manage memory and resources automatically, ensuring safety and preventing issues like memory leaks and dangling pointers. Examples include `Box<T>`, `Rc<T>`, and `Arc<T>`.
    they are smart because they smartly free memory when it goes out of scope, this is done by the `Drop` trait, which allows them to run custom code when they are dropped (go out of scope). This means that when a smart pointer is no longer needed, it can automatically clean up the memory it owns, preventing memory leaks and ensuring efficient memory management.
    * Q2: What is Box<T> in Rust?
    - A2: `Box<T>` is a smart pointer in Rust that allocates data on the heap. It is used when you want to store data that is too large to fit on the stack or when you want to have a single ownership of the data. `Box<T>` provides a way to store data on the heap while still allowing you to work with it as if it were on the stack. It is often used for recursive data structures or when you want to transfer ownership of data without copying it.
    - for example here our Vec<Box<u32>> is holding a list of addresses to heap-allocated `u32` values, and we can easily convert it back to a Vec<u32> by dereferencing the boxed values.
    - if we have 1GB boxed data, we can move the `Box<T>` around very cheaply because we are moving 8 bytes (the pointer) instead of 1GB of data. This is one of the key advantages of using `Box<T>` for large data structures.
    - if we have a recursive data structure, such as a linked list or a tree, rust needs to know the size of the data at compile time. Since recursive data structures can have an infinite size, we can use `Box<T>` to break the recursion and allow Rust to determine the size of the data at compile time. This is because `Box<T>` is a pointer, and its size is known (the size of a pointer), regardless of the size of the data it points to.
    * Q3: What is the difference between Box<T> and Rc<T> in Rust?
    - A3: The main difference between `Box<T>` and `Rc<T>` in Rust is that `Box<T>` provides single ownership of the data it points to, while `Rc<T>` provides shared ownership. `Box<T>` is used when you want to have a single owner of the data, and it will automatically deallocate the memory when the owner goes out of scope. On the other hand, `Rc<T>` allows multiple owners of the same data, and it uses reference counting to keep track of how many owners there are. When the last owner goes out of scope, the memory is deallocated.
    * Q4: What is the difference between Rc<T> and Arc<T> in Rust?
    - A4: The main difference between `Rc<T>` and `Arc<T>` in Rust is that `Rc<T>` is not thread-safe, while `Arc<T>` is thread-safe. `Rc<T>` uses reference counting to manage shared ownership of data, but it does not use atomic operations, which means it cannot be safely shared across multiple threads. On the other hand, `Arc<T>` uses atomic reference counting, which allows it to be safely shared across multiple threads. If you need to share data between threads, you should use `Arc<T>`, while if you only need to share data within a single thread, `Rc<T>` may be sufficient.
    so in summary, Arc<T> can track references across threads, while Rc<T> cannot. This makes Arc<T> suitable for concurrent programming, while Rc<T> is more suitable for single-threaded scenarios.
    also for the data races, Arc<T> uses atomic operations to manage the reference count, which ensures that it can safely handle concurrent access from multiple threads without causing data races. 
    * Q5: What is the drop trait in Rust and how does it relate to smart pointers?
    - A5: The `Drop` trait in Rust is a special trait that allows you to specify code that should be run when a value goes out of scope. This is particularly important for smart pointers, as it allows them to automatically clean up resources when they are no longer needed. When a smart pointer goes out of scope, the `Drop` trait is called, and it can perform any necessary cleanup, such as deallocating memory or closing file handles. This is a key feature of smart pointers in Rust, as it helps to ensure that resources are managed safely and efficiently without the need for manual memory management. For example, when a `Box<T>` goes out of scope, its `Drop` implementation will automatically deallocate the memory it owns, preventing memory leaks and ensuring that resources are properly cleaned up. This is one of the reasons why smart pointers are considered "smart", as they can automatically manage resources and prevent common issues like memory leaks and dangling pointers.
*/
