use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    file.write_all(content.as_bytes()).unwrap();
}

/*
    * Q & A :
    * Q1 : What is the `OpenOptions` struct and how does it work in Rust?
    - A1 : The `OpenOptions` struct in Rust is a builder pattern struct used to configure how a file should be opened. It is part of the `std::fs` module and provides a way to specify various options for opening a file, such as whether to create the file if it does not exist, whether to append to the file instead of overwriting it, and whether to open the file for reading, writing, or both. The `OpenOptions` struct has several methods that allow you to set these options, such as `create`, `append`, `read`, and `write`. Once you have configured the desired options, you can call the `open` method with a file path to attempt to open the file according to the specified options.
    * Q2 : What does the `unwrap` method do in Rust, and when should it be used?
    - A2 : The `unwrap` method in Rust is used to extract the value from an `Option` or `Result` type. When called on an `Option`, it will return the contained value if it is `Some`, but will panic if it is `None`. When called on a `Result`, it will return the contained value if it is `Ok`, but will panic if it is `Err`. The `unwrap` method should be used when you are certain that the value will be present and that it is acceptable for the program to panic if it is not. However, it is generally recommended to handle errors more gracefully using methods like `expect`, `match`, or `?` instead of using `unwrap`, as it can lead to more robust and maintainable code.
    * Q3 : What is the purpose of the `write_all` method in Rust's `Write` trait, and how does it differ from other writing methods?
    - A3 : The `write_all` method in Rust's `Write` trait is used to write an entire buffer of data to a writer. It takes a byte slice as an argument and attempts to write all of the bytes to the underlying writer. If the write operation is successful, it returns `Ok(())`. If an error occurs during the write operation, it returns an `Err` with the error information. The `write_all` method differs from other writing methods, such as `write`, in that it ensures that all bytes are written before returning. The `write` method may write only a portion of the data and return the number of bytes written, while `write_all` will continue to write until all bytes have been written or an error occurs. This makes `write_all` a more convenient choice when you want to ensure that all data is written without having to manually check the number of bytes written and handle partial writes.
    * Q4 : What does the &P type parameter in the `open_or_create` function signify, and how does it work with the `AsRef<Path>` trait?
    - A4 : The `&P` type parameter in the `open_or_create` function signifies that the function takes a reference to a generic type `P`. The `AsRef<Path>` trait bound on `P` indicates that `P` can be any type that can be converted to a `Path` reference. This allows the function to accept various types of path-like arguments, such as `&str`, `String`, or `PathBuf`, as long as they can be converted to a `Path`. Because &str, String, and PathBuf all implement the `AsRef<Path>` trait.
    * Q5 : How does the following work together? `<P: AsRef<Path>>(path: &P, content: &str) in simple terms?`
    - A5 : The `<P: AsRef<Path>>` part specifies that the function is generic over a type `P`, it means that the function can accept any type `P` as long as it implements the `AsRef<Path>` trait.
        and by doing so, Rust is making sure to prevent any type that cannot be converted to a `Path` from being used as an argument for the `path` parameter.
    * Q6 : What is the purpose of the `create(true)` and `append(true)` options in the `OpenOptions` configuration, and how do they affect file handling?
    - A6 : The `create(true)` option in the `OpenOptions` configuration tells Rust to create the file if it does not already exist. If the file already exists, this option has no effect. The `append(true)` option tells Rust to open the file in append mode.
    * Q7 : What does `read_to_string` do in Rust, and how does it differ from other file reading methods?
    - A7 : The `read_to_string` function in Rust is a convenience function that reads the entire contents of a file into a `String`. It takes a file path as an argument and returns a `Result<String, std::io::Error>`. If the file is successfully read, it returns `Ok(String)` containing the file's contents. If an error occurs, it returns `Err` with the error information. The `read_to_string` function differs from other file reading methods, such as `read`, which reads a specified number of bytes into a buffer and returns the number of bytes read. `read_to_string` abstracts away the details of reading the file in chunks and handles the entire reading process, making it easier to read the full contents of a file into a string without having to manage buffers or handle partial reads.
*/