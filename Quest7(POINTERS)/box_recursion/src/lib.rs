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
            Some(target ) => {
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
