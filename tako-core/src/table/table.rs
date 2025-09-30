use serde::{Deserialize, Serialize};

// 或者可以考虑FixTaskRow的表述，后续增加CustomTaskRow
// 二者组合形成最终的task table
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskRow {
    pub id: u32,
    pub title: String,
    pub status: Radio,
    pub active: u32,
    pub due: String,
    pub priority: Radio,
    pub feat: Radio,
    pub tags: MultiSelect,
    pub age: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Radio {

}

// FixRadio and MutRadio
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MutRadio {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiSelect {

}
