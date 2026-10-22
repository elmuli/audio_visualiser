use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

<<<<<<< HEAD
#[derive(Default)]
pub struct App{
    pub should_quit: bool,
}
mod app;
mod audio;


fn main() -> std::io::Result<()> {

    let mut app = app::App{
        should_quit: false,
        buf: [0i16; 1024],
    };


    audio::read_audio(&mut app);

    ratatui::run(|mut terminal| {
        loop {
            let text = format!("{:?}", app.buf);
            terminal.draw(|frame| frame.render_widget(text, frame.area()))?;
            handle_events(&mut app);
            if app.should_quit {
                break Ok(());
            }
        }
    })
}

fn handle_events(app: &mut app::App) -> std::io::Result<bool>{
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code{
            KeyCode::Char('q') => app.should_quit = true, _ => {}
        },
        _ => {}
    }
    Ok(false)
=======


fn main() {
    print!("Hello world");
>>>>>>> 4e71c79 (test)
}
