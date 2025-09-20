use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::Pane;

/// 负责一帧的绘制
pub fn draw(frame: &mut Frame<'_>, focused: Pane) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(frame.size());

    // 左 Pane
    let left_title = match focused {
        Pane::Left => "Left (focused)",
        _ => "Left" 
    };
    let left_style = match focused { 
        Pane::Left => Style::default().fg(Color::Yellow),
        _ => Style::default() 
    };
    let left_block = Block::default()
        .title(left_title)
        .borders(Borders::ALL)
        .style(left_style);
    frame.render_widget(left_block, chunks[0]);

    // （示例）左 Pane 内放一点占位文本
    let left_text = Paragraph::new("这里将来显示任务列表…\n(j/k 上下, 空格选择, etc.)");
    frame.render_widget(left_text, inner(chunks[0]));

    // 右 Pane
    let right_title = match focused { Pane::Right => "Right (focused)", _ => "Right" };
    let right_style = match focused { Pane::Right => Style::default().fg(Color::Yellow), _ => Style::default() };
    let right_block = Block::default()
        .title(right_title)
        .borders(Borders::ALL)
        .style(right_style);
    frame.render_widget(right_block, chunks[1]);

    let right_text = Paragraph::new("这里将来显示任务详情或日志…");
    frame.render_widget(right_text, inner(chunks[1]));
}

/// 给块内部留出一点内边距（不紧贴边框）
fn inner(area: ratatui::prelude::Rect) -> ratatui::prelude::Rect {
    Layout::default()
        .margin(1)
        .constraints([Constraint::Percentage(100)])
        .split(area)[0]   // ← 直接取第 0 个
}
