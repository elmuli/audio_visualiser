use ratatui::Frame;
use ratatui::widgets::canvas::{Canvas, Line, Rectangle, Points};
use ratatui::style::Color;

use crate::app;

pub fn Render(frame: &mut Frame, app: &mut app::App){
    let text = format!("{:?}", app.buf);
}

fn draw_lines(frame: &mut Frame, app: &mut app::App){
    let canvas = Canvas::default().paint(|ctx|{
        for i in 0..10{
            ctx.draw(&Line::new(i as f64, 0.0, i as f64, app.buf[i].into(), Color::Blue));
        }
    });
}
