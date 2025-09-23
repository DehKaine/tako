use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::app::commands;
use tako_widgets::{UiState, Pane, draw as draw_ui};

pub struct App;

impl App {
    pub async fn serve() -> Result<()> {
        // 进入 TUI 模式
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut term = Terminal::new(backend)?;

        let mut state = UiState::default();
        let mut running = true;

        // 首帧
        term.draw(|f| draw_ui(f, &state))?;

        while running {
            // 简洁轮询；将来可换 tokio::select! 合并定时/IO 事件（yazi 也是异步驱动）  [oai_citation:2‡yazi-rs.github.io](https://yazi-rs.github.io/?utm_source=chatgpt.com)
            if event::poll(std::time::Duration::from_millis(120))? {
                if let Event::Key(k) = event::read()? {
                    if k.kind == KeyEventKind::Press {
                        match k.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                commands::quit::run(&mut running)?;
                            }
                            KeyCode::Tab => {
                                state.focused = match state.focused {
                                    Pane::Left => Pane::Right,
                                    Pane::Right => Pane::Left,
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }

            // 统一重绘（后续可做“有状态变化才画”的节流）
            term.draw(|f| draw_ui(f, &state))?;
        }

        // 退出清理
        disable_raw_mode()?;
        execute!(term.backend_mut(), LeaveAlternateScreen)?;
        term.show_cursor()?;
        Ok(())
    }
}
