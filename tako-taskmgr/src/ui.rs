use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    style::Style,
    Frame,
};

pub fn draw(frame: &mut Frame<'_>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.size());

    let left  = Block::default().title("Tasks").borders(Borders::ALL);
    let right = Block::default().title("Task Info").borders(Borders::ALL);

    frame.render_widget(left, chunks[0]);
    frame.render_widget(right, chunks[1]);

    // 你可以在右侧放一个占位说明
    frame.render_widget(Paragraph::new("Press q / Esc to quit"), chunks[1]);
}
