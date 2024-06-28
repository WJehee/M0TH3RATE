use ratatui::{prelude::*, widgets::*};

#[derive(Debug)]
pub struct MyGauge {
    name: String,
    value: f64,
    color: Color,
}

impl MyGauge {
    pub fn new(name: &str, value: f64, color: Color) -> Self {
        Self {
            name: name.to_string(),
            value,
            color,
        }
    }
}

impl Widget for &MyGauge {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //let gauge = Gauge::default()
        //    .block(Block::bordered().title(self.name.clone()))
        //    .gauge_style(
        //        Style::default()
        //        .fg(self.color)
        //        .bg(Color::Black)
        //        .add_modifier(Modifier::ITALIC),
        //    )
        //    .percent(self.value);
        //gauge.render(area, buf);
        
        let line_gauge = LineGauge::default()
            .block(Block::bordered().title(self.name.clone()))
            .filled_style(
                Style::default()
                .fg(self.color)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
            )
            .line_set(symbols::line::THICK)
            .ratio(self.value);
        line_gauge.render(area, buf);
    }
}

