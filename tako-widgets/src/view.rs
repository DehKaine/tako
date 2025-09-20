use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pane { Left, Right }

#[derive(Debug)]
pub struct UiState {
    pub focused: Pane,
    // 这里以后补：任务集合、选中行、滚动偏移等
}

impl Default for UiState {
    fn default() -> Self { Self { focused:Pane::Left } }
}

pub fn draw(f: &mut Frame<'_>, st: &UiState) {
    // 垂直：主区 + 一行提示
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area()); // <- size() 改成 area()

    let main   = chunks[0];
    let footer = chunks[1];

    // 主区水平 7:3
    let lr = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main);

    let hl = Style::default().fg(Color::Yellow);
    let left_style  = if st.focused == Pane::Left  { hl } else { Style::default() };
    let right_style = if st.focused == Pane::Right { hl } else { Style::default() };

    f.render_widget(Block::default().borders(Borders::ALL).title("Tasks").style(left_style),  lr[0]);
    f.render_widget(Block::default().borders(Borders::ALL).title("Task Info").style(right_style), lr[1]);

    f.render_widget(Paragraph::new("Tab 切换焦点 · q/Esc 退出"), footer);
}
