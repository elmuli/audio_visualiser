use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect, Margin};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, Paragraph, Clear};
use ratatui::widgets::canvas::{Canvas, Line, Rectangle};

use crate::app;
use crate::audio;

pub fn render(frame: &mut Frame, app: &mut app::App){
    let _text = format!("{:?}", app.buf);
    use Constraint::{Fill, Length, Min};

    let [text_area, visualiser] = Layout::vertical([Length(1), Min(0)]).areas(frame.area());

    audio::read_audio(app);
    frame.render_widget(
        Canvas::default()
        .block(Block::bordered().title("Visualiser"))
        .paint(|ctx|{
            let mut i = 0;
            let mut value = 0;
            while i<2048 {
                value += app.buf[i];

                if i%128 == 0{
                    value /= 16;
                    ctx.draw(&Rectangle {
                        x: i as f64,
                        y: 100.0,
                        width: 128.0,
                        height: value as f64,
                        color: Color::Blue,
                    });
                    value = 0;
                }

                i+=1;
            }
        })
        .x_bounds([0.0, 4048.0])
        .y_bounds([0.0, 200.0]), 
        visualiser
    );
}

