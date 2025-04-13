use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct List {
    pub categories: Vec<Category>,
}

impl List {
    pub fn write(&mut self, path: &Path, new_file: bool) -> anyhow::Result<()> {
        if new_file {
            *self = Self::default();
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json).map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    pub fn load(&mut self, path: &Path) -> anyhow::Result<()> {
        let json = std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!(e.to_string()))?;

        *self = serde_json::from_str(&json).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Category {
    pub name: String,
    pub items: Vec<Item>,
    #[serde(default = "new_uuid")]
    pub id: uuid::Uuid,
}

impl Default for Category {
    fn default() -> Self {
        Self {
            name: String::new(),
            items: Vec::new(),
            id: new_uuid(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub todo: bool,
    #[serde(default)]
    pub notes: String,
    #[serde(default = "new_uuid")]
    pub id: uuid::Uuid,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: String::new(),
            todo: false,
            notes: String::new(),
            id: new_uuid(),
        }
    }
}

fn new_uuid() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}
