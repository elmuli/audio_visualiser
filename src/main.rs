use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

mod app;
mod audio;
mod ui;

fn main() -> std::io::Result<()> {

    let mut app = app::App{
        should_quit: false,
        buf: [0i16; 1024],
    };

    audio::read_audio(&mut app);

    ratatui::run(|mut terminal| {
        loop {
            handle_events(&mut app);
            terminal.draw(|frame| ui::Render(frame, &mut app))?;
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
}
