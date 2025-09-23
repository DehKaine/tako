use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

// ★ 加这一行：把 Stylize 引入作用域，才有 .bold()
use ratatui::prelude::Stylize;

// ============== UI 状态（放在 widgets 里集中管理渲染需要的状态） ==============

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Pane {
    #[default] Left,   // ★ 给枚举一个默认分支，UiState 才能派生 Default
    Right,
}

#[derive(Debug, Default)]
pub struct UiState {
    pub focused: Pane,
    pub row: usize,
    pub col: usize,
    pub editing: bool,
    pub input: String, // 编辑缓冲
}

// 外部（fm）传入的只读数据（渲染所需）
pub struct TableViewData {
    pub headers: &'static [&'static str],
    pub rows: Vec<Vec<String>>,
}

pub fn draw(f: &mut Frame<'_>, ui: &UiState, data: &TableViewData) {
    // 上主区 + 下状态行
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area()); // 0.29 用 area()

    // 左右 7:3
    let lr = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[0]);

    // 表头
    let header = Row::new(
        data.headers
            .iter()
            .map(|h| Cell::from(Line::from(*h).bold()))
    );

    // 行 + 单元格：光标所在 cell 高亮
    let rows = data.rows.iter().enumerate().map(|(r_idx, row)| {
        let cells = row.iter().enumerate().map(|(c_idx, s)| {
            let mut cell = Cell::from(s.as_str());
            if ui.row == r_idx && ui.col == c_idx {
                cell = cell.style(
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                );
            }
            cell
        });
        Row::new(cells)
    });

    // 列宽（先定长；以后可换成 Min/Max/Percentage 混合）
    let widths: Vec<Constraint> = data.headers.iter().map(|_| Constraint::Length(12)).collect();

    // 行选中高亮（注意：TableState 只管“行”）
    let mut state = TableState::default();
    state.select(Some(ui.row));
    let row_hl = Style::default().bg(Color::Blue).fg(Color::White);

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(row_hl) // ★ 新 API：替代 highlight_style
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tasks (hjkl 移动, i 编辑, o 新建)")
        );

    f.render_stateful_widget(table, lr[0], &mut state);

    // 右侧面板（先占位）
    let right = Block::default()
        .borders(Borders::ALL)
        .title(match ui.focused { Pane::Left => "Task Info", Pane::Right => "Task Info*" });
    f.render_widget(right, lr[1]);

    // 底部状态行
    let footer = Block::default().borders(Borders::NONE).title(if ui.editing {
        format!("INSERT: {}", ui.input)
    } else {
        "Tab 切换 · q 退出 · o 新行 · i 编辑 · Enter 提交 · Esc 取消".to_string()
    });
    f.render_widget(footer, chunks[1]);
}
