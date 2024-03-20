use anyhow::Context;
use serde::{Deserialize, Serialize};

pub struct Store {
    // what does here??
    items: Vec<ToDo>,
}

#[derive(Deserialize, Serialize)]
pub struct ToDo {
    id: usize,
    title: String,
    slug: String,
    done: Option<()>,
}

impl Store {
    pub fn from_fs(location: &str) -> Result<Self, anyhow::Error> {
        let raw = std::fs::read_to_string(location)
            .with_context(|| format!("read todo json file: {location}"))?;

        let todos: Vec<ToDo> = serde_json::from_str(&raw).context("parse todo json file")?;

        Ok(Store { items: todos })
    }

    pub fn format_todos(&self) -> String {
        if self.items.is_empty() {
            return "No ToDos found - have a lovely day!".to_string();
        }
        let mut out = String::new();
        self.items
            .iter()
            .for_each(|item| out.push_str(&format!("ID: {} -- Title: {}", item.id, &item.title)));

        out
    }
}
