use crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod app;
mod audio;
mod ui;

fn main() -> std::io::Result<()> {

    let mut app = app::App{
        should_quit: false,
        buf: [0i16; 2048],
    };


    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| ui::render(frame, &mut app))?;
            let _ = handle_events(&mut app);
            if app.should_quit {
                break Ok(());
            }
        }
    })
}

fn handle_events(app: &mut app::App) -> std::io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(10))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(app.should_quit)
}
