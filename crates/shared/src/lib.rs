use serde::{Deserialize, Serialize};

pub mod ipc;
pub mod settings;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WindowInfo {
    pub id: u64,
    pub app_id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettingValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}
