mod app;
mod auth;
mod ui;
mod handlers;
mod events;

use app::App;

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::init();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;
        if events::handle(&mut app)? {
            break Ok(());
        }
    }
}
