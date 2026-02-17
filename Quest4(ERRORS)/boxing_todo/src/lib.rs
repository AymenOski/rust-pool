mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let contents = fs::read_to_string(path).map_err(|e| ReadErr {
            child_err: Box::new(e),
        })?;

        let contents = json::parse(&contents).map_err(|e| ParseErr::Malformed(Box::new(e)))?;
        if contents["tasks"].is_empty() {
            return Err(ParseErr::Empty.into());
        }

        Ok(Self {
            title: contents["title"].as_str().unwrap().to_owned(),
            tasks: contents["tasks"]
                .members()
                .map(|m| Task {
                    id: m["id"].as_u32().unwrap(),
                    description: m["description"].as_str().unwrap().to_owned(),
                    level: m["level"].as_u32().unwrap(),
                })
                .collect(),
        })
    }
}

/*
    * Q & A:
    * Q1 : What is Box<T> for?
    - A1 : Box<T> is a smart pointer in Rust that allows you to store data on the heap instead of the stack. It provides ownership and automatic memory management for the data it points to.
    how it works:
    - When you create a Box<T>, it allocates memory on the heap for the data of type T and returns a pointer to that memory. The Box<T> itself is stored on the stack, but the data it points to is on the heap. This allows you to store large amounts of data without worrying about stack overflow, as the heap can grow dynamically. Additionally, Box<T> provides ownership semantics, meaning that when a Box<T> goes out of scope, the memory it points to is automatically deallocated, preventing memory leaks.
    - In an ideal world, everything should be stored on the stack for performance reasons, but in practice, we often need to store data that is too large or has a size that cannot be determined at compile time. In such cases, we use Box<T> to store the data on the heap while still maintaining ownership and memory safety.
    - also the compiler have to know the size of the type at compile time, but when we want to return a trait object (like Box<dyn Error>), we can't know the size of the concrete type that implements the trait at compile time. By using Box<dyn Error>, we can store any type that implements the Error trait on the heap and return a pointer to it.
*/