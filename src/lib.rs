mod app;
mod events;
mod handlers;
mod soundcloud;
mod ui;

use app::App;
use soundcloud::client::Client;
use std::thread;

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    //make arc mutex for client and then pass it to app so can use it
    let handle = thread::spawn(|| {
        Client::init();
    });

    let mut app = App::init();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;
        if events::handle(&mut app)? {
            break Ok(());
        }
    }
}
