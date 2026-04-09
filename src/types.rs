use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ListItem {
    pub id: String,
    pub tip: String,
    pub hint: String,
    pub is_folded: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Part {
    pub id: String,
    pub name: String,
    pub items: Vec<ListItem>,
}

#[derive(PartialEq)]
pub enum Mode { Read, Edit }
