use std::collections::VecDeque;

use ratatui::{
    prelude::*,
    style::Color,
    symbols::border,
    widgets::{Block, Paragraph, StatefulWidget, Wrap},
};
use throbber_widgets_tui::{Throbber, ThrobberState};

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Info,
    Warning,
    Error,
    Critical,
}

impl Level {
    pub fn color(&self) -> Color {
        match self {
            Level::Info => Color::Cyan,
            Level::Warning => Color::Yellow,
            Level::Error => Color::Magenta,
            Level::Critical => Color::Red,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Level::Info => "INFO",
            Level::Warning => "WAARSCHUWING",
            Level::Error => "FOUT",
            Level::Critical => "KRITIEK",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Level::Info => "⊕",
            Level::Warning => "⊞",
            Level::Error => "!",
            Level::Critical => "●",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub level: Level,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct Notifications {
    queue: VecDeque<Notification>,
}

impl Notifications {
    pub fn push(&mut self, level: Level, message: impl Into<String>) {
        self.queue.push_back(Notification {
            level,
            message: message.into(),
        });
    }

    pub fn dismiss(&mut self) {
        self.queue.pop_front();
    }
}

impl StatefulWidget for &Notifications {
    type State = ThrobberState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let Some(current) = self.queue.front() else {
            return;
        };

        let color = current.level.color();
        let pending = self.queue.len();
        let mut title_spans: Vec<Span<'_>> = Vec::with_capacity(5);
        title_spans.push(" ".into());
        if !matches!(current.level, Level::Critical) {
            title_spans.push(current.level.icon().bold().fg(color));
            title_spans.push(" ".into());
        }
        title_spans.push(current.level.label().bold().fg(color));
        title_spans.push(if pending > 1 {
            format!(" ({} in wachtrij) ", pending).into()
        } else {
            " ".into()
        });
        let title = Line::from(title_spans);

        let block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center)
            .border_set(border::THICK)
            .border_style(Style::default().fg(color));
        let inner = block.inner(area);
        block.render(area, buf);

        if matches!(current.level, Level::Critical) {
            let throbber = Throbber::default()
                .label(current.message.clone())
                .style(Style::default().fg(color))
                .throbber_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
                .throbber_set(throbber_widgets_tui::BLACK_CIRCLE)
                .use_type(throbber_widgets_tui::WhichUse::Spin);
            StatefulWidget::render(throbber, inner, buf, state);
        } else {
            Paragraph::new(current.message.clone())
                .wrap(Wrap { trim: true })
                .render(inner, buf);
        }
    }
}
