mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
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
    let content = fs::read_to_string(path).map_err(|e| {
        Box::new(ReadErr { child_err: Box::new(e) })
    })?;

    // Parse JSON (Wrapped in ParseErr::Malformed)
    let parsed = json::parse(&content).map_err(|e| {
        Box::new(ParseErr::Malformed(Box::new(e)))
    })?;

    // Safety Check: If the JSON is valid but doesn't have the expected structure, 
    // we should treat it as Malformed or Empty.
    if parsed["tasks"].is_empty() {
        return Err(Box::new(ParseErr::Empty));
    }

    // Use safe extraction instead of immediate unwraps where possible
    let title = parsed["title"]
        .as_str()
        .ok_or_else(|| Box::new(ParseErr::Malformed("Missing title".into())))?
        .to_string();

    let tasks: Vec<Task> = parsed["tasks"]
        .members()
        .filter_map(|m| {
            let id = m["id"].as_u32()?;
            let description = m["description"].as_str()?.to_string();
            let level = m["level"].as_u32()?;
            Some(Task { id, description, level })
        })
        .collect();

    Ok(TodoList { title, tasks })
}
}