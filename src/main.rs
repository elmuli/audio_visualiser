use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

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

    /*
    ratatui::run(|mut terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget("Hello World!", frame.area()))?;
            handle_events(&mut app);
            if app.should_quit {
                break Ok(());
            }
        }
    })*/

    Ok(())
}

fn handle_events(app: &mut app::App) -> std::io::Result<bool>{
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code{
            KeyCode::Char('q') => app.should_quit = true, _ => {}
        },
        _ => {}
    }
    Ok(false)
}
