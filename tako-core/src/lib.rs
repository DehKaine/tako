use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskRow {
    pub id: String,
    pub title: String,
    pub status: String,
    pub active: String,
    pub due: String,
    pub priority: String,
    pub feat: String,
    pub tags: String,
    pub age: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskTable {
    pub rows: Vec<TaskRow>,
}

fn data_file() -> Result<PathBuf> {
    let proj = ProjectDirs::from("dev", "tako", "tako") // org, app
        .ok_or_else(|| anyhow::anyhow!("no project dirs"))?;
    let data_dir = proj.data_dir();
    fs::create_dir_all(data_dir)?;
    Ok(data_dir.join("tasks.json"))
}

pub fn load_table() -> Result<TaskTable> {
    let path = data_file()?;
    if !path.exists() {
        // 初次运行给一个空表 + 一行空行也行：rows: vec![TaskRow::default()]
        return Ok(TaskTable::default());
    }
    let s = fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?;
    let table: TaskTable = serde_json::from_str(&s)
        .with_context(|| format!("parse {}", path.display()))?;
    Ok(table)
}

pub fn save_table(table: &TaskTable) -> Result<()> {
    let path = data_file()?;
    let s = serde_json::to_string_pretty(table)?;
    fs::write(&path, s)
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
