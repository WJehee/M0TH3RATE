use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
};

use ratatui::prelude::*;

use crate::{components::ship_status::MyGauge, tui};


#[derive(Debug)]
enum Selected {
    Status,
}

#[derive(Debug)]
pub struct App {
    selected: Selected,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected: Selected::Status,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(frame.size());

        let gauges = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(chunks[1]);

        frame.render_widget(self, chunks[0]);
        frame.render_widget(&MyGauge::new("Shields", 0.9, Color::Blue), gauges[0]);
        frame.render_widget(&MyGauge::new("Power", 0.5, Color::Yellow), gauges[1]);
        frame.render_widget(&MyGauge::new("Fuel", 0.3, Color::Red), gauges[2]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_press_event(key);
            }
        }
        Ok(())
    }

    fn handle_press_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q')  => { self.exit = true; },
            KeyCode::Up         => {},
            KeyCode::Down       => {},
            _ => {},
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
                " Quit ".into(),
                "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec![
                "Hello world".into(),
        ])]);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

