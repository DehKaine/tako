use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use tako_core::{TaskRow, TaskTable, load_table, save_table};
use tako_widgets::{UiState, Pane, TableViewData, draw};

const COLS: [&str; 9] = ["ID","Title","Status","Active","Due","Priority","Feat","Tags","Age"];

pub struct App;

impl App {
    pub async fn serve() -> Result<()> {
        // --- TUI init ---
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut term = Terminal::new(backend)?;

        // --- 状态 ---
        let mut ui = UiState::default();
        let mut table = load_or_seed()?; // 读 JSON，没有则给一行空
        let mut running = true;

        // 首帧
        term.draw(|f| draw(f, &ui, &to_view(&table)))?;

        // 主循环
        while running {
            // 简洁轮询；将来可合 tokio 定时器/消息
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(k) = event::read()? {
                    handle_key(k, &mut ui, &mut table, &mut running)?;
                }
            }
            term.draw(|f| draw(f, &ui, &to_view(&table)))?;
        }

        // 退出前保存
        save_table(&table).ok();

        // 收尾
        disable_raw_mode()?;
        execute!(term.backend_mut(), LeaveAlternateScreen)?;
        term.show_cursor()?;
        Ok(())
    }
}

fn load_or_seed() -> Result<TaskTable> {
    let mut t = load_table()?;
    if t.rows.is_empty() {
        t.rows.push(TaskRow::default());
    }
    Ok(t)
}

// 将业务数据映射成 widgets 需要的视图矩阵
fn to_view(t: &TaskTable) -> TableViewData {
    let rows = t.rows.iter().map(|r| {
        vec![
            r.id.clone(), r.title.clone(), r.status.clone(), r.active.clone(),
            r.due.clone(), r.priority.clone(), r.feat.clone(), r.tags.clone(), r.age.clone(),
        ]
    }).collect::<Vec<_>>();
    TableViewData { headers: &COLS, rows}
}

fn handle_key(k: KeyEvent, ui: &mut UiState, table: &mut TaskTable, running: &mut bool) -> Result<()> {
    // 只处理按下事件（crossterm KeyEvent 有 is_press/is_release 区分） [oai_citation:4‡Docs.rs](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html?utm_source=chatgpt.com)
    use KeyCode::*;
    if k.kind.is_press() {
        match (ui.editing, k.code) {
            // --- 编辑模式 ---
            (true, Char(c)) if c != '\n' => { ui.input.push(c); }
            (true, Backspace) => { ui.input.pop(); }
            (true, Enter) => { write_cell(ui, table); ui.editing = false; save_table(table).ok(); }
            (true, Esc) =>   { ui.input.clear(); ui.editing = false; }
            (true, _) => {}

            // --- 普通模式 ---
            (false, Char('q')) | (false, Esc) => { *running = false; }
            (false, Char('o')) => { table.rows.push(TaskRow::default()); ui.row = table.rows.len()-1; ui.col = 0; save_table(table).ok(); }
            (false, Char('i')) => { ui.editing = true; ui.input = read_cell(ui, table); }
            (false, Char('h')) => { if ui.col > 0 { ui.col -= 1; } }
            (false, Char('l')) => { if ui.col + 1 < COLS.len() { ui.col += 1; } }
            (false, Char('k')) => { if ui.row > 0 { ui.row -= 1; } }
            (false, Char('j')) => { if ui.row + 1 < table.rows.len() { ui.row += 1; } }
            _ => {}
        }
    }
    Ok(())
}

fn read_cell(ui: &UiState, t: &TaskTable) -> String {
    let r = &t.rows[ui.row];
    match ui.col {
        0 => r.id.clone(),
        1 => r.title.clone(),
        2 => r.status.clone(),
        3 => r.active.clone(),
        4 => r.due.clone(),
        5 => r.priority.clone(),
        6 => r.feat.clone(),
        7 => r.tags.clone(),
        8 => r.age.clone(),
        _ => String::new(),
    }
}

fn write_cell(ui: &UiState, t: &mut TaskTable) {
    let r = &mut t.rows[ui.row];
    let v = ui.input.clone();
    match ui.col {
        0 => r.id = v,
        1 => r.title = v,
        2 => r.status = v,
        3 => r.active = v,
        4 => r.due = v,
        5 => r.priority = v,
        6 => r.feat = v,
        7 => r.tags = v,
        8 => r.age = v,
        _ => {}
    }
}
