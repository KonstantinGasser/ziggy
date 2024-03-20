use anyhow::Context;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Store {
    items: Vec<ToDo>,
}

#[derive(Deserialize, Serialize)]
pub struct ToDo {
    id: String,
    title: String,
    slug: String,
    done: bool,
}

impl ToDo {
    pub fn new(title: String) -> Self {
        ToDo {
            id: Uuid::new_v4().to_string(),
            title: title,
            slug: String::from("Mhm, what can go here?"),
            done: false,
        }
    }
}

impl Store {
    pub fn from_fs(location: &str) -> Result<Self, anyhow::Error> {
        let raw = std::fs::read_to_string(location)
            .with_context(|| format!("read todo json file: {location}"))?;

        let todos = serde_json::from_str::<Vec<ToDo>>(&raw).context("parse todo json file")?;

        Ok(Store { items: todos })
    }

    // NOTE: adding a todo does not automatically save the todos to local storage.
    // this needs to be done in a following step.
    pub fn add_todo(&mut self, td: ToDo) {
        self.items.push(td);
    }

    pub fn write_fs(&self, location: &str) -> Result<(), anyhow::Error> {
        let pretty_bytes =
            serde_json::to_vec_pretty(&self.items).context("write ToDo items to local storage")?;

        std::fs::write(location, pretty_bytes.as_slice())
            .with_context(|| format!("write ToDo items to json file: {location}"))?;
        Ok(())
    }

    pub fn format_todos(&self) -> String {
        if self.items.is_empty() {
            return "No ToDos found - have a lovely day!".to_string();
        }

        let mut out = String::new();
        self.items.iter().for_each(|item| {
            out.push_str(&format!(
                "ID: {} -- Title: {} -- Done: {}",
                item.id, &item.title, item.done
            ))
        });

        out
    }
}
