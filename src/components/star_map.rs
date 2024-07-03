use ratatui::{prelude::*, widgets::canvas::{Canvas, Circle, Context, Line, Rectangle}};

struct Location {
    x: f64,
    y: f64,
    radius: f64,
    color: Color,
}

impl Location {
    pub fn draw(&self, ctx: &mut Context, highlighted: Option<Color>) {
        ctx.draw(&Circle {
            x: self.x,
            y: self.y,
            radius: self.radius,
            color: self.color,
        });
        ctx.draw(&Circle {
            x: self.x,
            y: self.y,
            radius: self.radius * 1.7,
            color: highlighted.unwrap_or(Color::DarkGray),
        });

        // Highlight (either animate ship or just star in the middle?)
        if let Some(highlight_color) = highlighted {
            ctx.draw(&Rectangle{
                x: self.x-0.5,
                y: self.y-0.5,
                width: 1.0,
                height: 1.0,
                color: highlight_color,
            });
        }
    }
}

pub struct StarMap {
    locations: Vec<Location>,
}

impl StarMap {
    pub fn new() -> Self {
        let mut locations = Vec::new();
        locations.push(Location {
            x: 50.0,
            y: 80.0,
            radius: 5.0,
            color: Color::Magenta,
        });
        locations.push(Location {
            x: 30.0,
            y: 20.0,
            radius: 8.0,
            color: Color::Red,
        });
        locations.push(Location {
            x: 80.0,
            y: 30.0,
            radius: 3.0,
            color: Color::LightGreen,
        });
        StarMap { locations }
    }
}

impl Widget for &StarMap {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
            .paint(|ctx| {
                // Draw each location
                for location in self.locations.iter() {
                    if location.radius == 5.0 {
                        location.draw(ctx, Some(Color::White));
                    } else {
                        location.draw(ctx, None);
                    }
                }
                // Draw lines between locations
                ctx.draw(&Line {
                    x1: 50.0,
                    y1: 80.0,
                    x2: 30.0,
                    y2: 20.0,
                    color: Color::DarkGray,
                });
                ctx.draw(&Line {
                    x1: 30.0,
                    y1: 20.0,
                    x2: 80.0,
                    y2: 30.0,
                    color: Color::DarkGray,
                });
                // TODO: highlight current position
                // TODO: hightlight selected position on link and on location
                // TODO: show box displaying info about location
            })
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .render(area, buf);
    }
}

