//* recommendation (⚠️): use `Colorful Comments` extension for better readability of the comments in this file7

#[derive(Debug, PartialEq, Clone , Copy)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let old_head = self.grade.take();
        
        let new_worker = Worker {
            role: Role::from(role),
            name : String::from(name),
            next :old_head,
        };

        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            Some(target) => {
                self.grade = target.next;
                Some(target.name)
            }
            None => None,
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        
        let first = self.grade.as_ref()?;
            Some((first.name.clone(), Role::from(first.role)))
    }
}

/*
    * Q & A : 
    * Q1 : Why do we use `Box` here?
    - A1 : We use `Box` to enable recursive data structures. In Rust, a type cannot directly contain itself, but it can contain a `Box` that points to the same type. This allows us to create a linked list-like structure where each `Worker` can point to the next `Worker` in the chain.
    * Q2 : What is the purpose of the `Option` type in the `Link` type alias?
    - A2 : The `Option` type is used to represent the possibility of a `Worker` being present or not. In this context, `Link` can either be `Some(Box<Worker>)` if there is a worker in that position, or `None` if there is no worker. This allows us to handle cases where the work environment might be empty or when we reach the end of the chain of workers.
    * Q3 : What does the `take()` method do?
    - A3 : The `take()` method is used to take the value out of an `Option`, leaving `None` in its place.
    * Q4 : Why do we do as_ref() in the last_worker method?
    - A4 : We use `as_ref()` to get a reference to the value inside the `Option` without taking ownership of it. This allows us to access the worker's name and role without consuming the `Option`, which is important because we want to keep the worker in the work environment for future operations.
*/