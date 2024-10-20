use ratatui::{backend::Backend, Terminal};

use crate::{game::GameResult, ui};

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    // pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }

    pub fn draw(&mut self) -> GameResult<()> {
        self.terminal.draw(|frame| ui::render(frame))?;
        Ok(())
    }
}
