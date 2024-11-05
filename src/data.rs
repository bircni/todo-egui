use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct List {
    pub categories: Vec<Category>,
}

impl List {
    pub fn new(json: Option<String>) -> anyhow::Result<Self> {
        json.map_or_else(
            || Ok(Self::default()),
            |json| serde_json::from_str(&json).map_err(|e| anyhow::anyhow!(e.to_string())),
        )
    }

    pub fn write(&self) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write("data.json", json).map_err(|e| anyhow::anyhow!(e.to_string()))
        // if result.is_err() {
        //     Ok("Failed to write data.json".to_owned())
        // } else {
        //     Ok("Data saved".to_owned())
        // }
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        let json =
            std::fs::read_to_string("data.json").map_err(|e| anyhow::anyhow!(e.to_string()))?;
        *self = serde_json::from_str(&json).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Category {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub todo: bool,
    #[serde(default)]
    pub notes: String,
}
