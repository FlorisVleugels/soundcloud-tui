mod ui;
mod handlers;
mod events;

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui::draw_main_layout(frame))?;
        if events::handle()? {
            break Ok(());
        }
    }
}
