use std::io::{self, Stdout};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

mod ui;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pane { Left, Right }

pub struct App {
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    focused: Pane,
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            terminal: None,
            focused: Pane::Left,
            running: true,
        }
    }

    /// 初始化终端、进入事件循环
    pub fn run(&mut self) -> Result<()> {
        self.enter_terminal()?;
        let res = self.event_loop(); // 事件循环里画 UI、处理键盘
        self.leave_terminal()?;      // 无论如何都要恢复终端
        res
    }

    fn event_loop(&mut self) -> Result<()> {
        while self.running {
            // 1) 绘制一帧
            if let Some(term) = &mut self.terminal {
                term.draw(|f| ui::draw(f, self.focused))?;
            }

            // 2) 处理输入（200ms 轮询）
            if event::poll(std::time::Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    // 只响应按下（忽略长按/重复）
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                            KeyCode::Tab => {
                                self.focused = match self.focused {
                                    Pane::Left => Pane::Right,
                                    Pane::Right => Pane::Left,
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn enter_terminal(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        self.terminal = Some(terminal);
        Ok(())
    }

    fn leave_terminal(&mut self) -> Result<()> {
        // 优雅恢复终端
        disable_raw_mode()?;
        if let Some(term) = &mut self.terminal {
            execute!(term.backend_mut(), LeaveAlternateScreen)?;
            term.show_cursor()?;
        }
        Ok(())
    }
}