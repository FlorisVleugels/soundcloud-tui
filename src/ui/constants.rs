use ratatui::layout::Constraint;

pub const HEADER_ASCII: &str = r#"
  _________                        .___     .__                   .___          __        .__ 
 /   _____/ ____  __ __  ____    __| _/____ |  |   ____  __ __  __| _/        _/  |_ __ __|__|
 \_____  \ /  _ \|  |  \/    \  / __ |/ ___\|  |  /  _ \|  |  \/ __ |  ______ \   __\  |  \  |
 /        (  <_> )  |  /   |  \/ /_/ \  \___|  |_(  <_> )  |  / /_/ | /_____/  |  | |  |  /  |
/_______  /\____/|____/|___|  /\____ |\___  >____/\____/|____/\____ |          |__| |____/|__|
        \/                  \/      \/    \/                       \/
"#;

pub const LIBRARY_ITEMS: [&str; 4] = ["Recently Played", "Liked Tracks", "Artists", "Albums"];

pub const MAIN_CONSTRAINTS: [Constraint; 3] = [
    Constraint::Length(3),
    Constraint::Min(0),
    Constraint::Length(6),
];
pub const TOP_BAR_CONSTRAINTS: [Constraint; 2] =
    [Constraint::Percentage(95), Constraint::Percentage(5)];
pub const BODY_CONSTRAINTS: [Constraint; 2] =
    [Constraint::Percentage(88), Constraint::Percentage(12)];
pub const BODY_BAR_CONSTRAINTS: [Constraint; 2] =
    [Constraint::Percentage(20), Constraint::Percentage(80)];

pub const TRACKS_COLUMN_WIDTHS: [Constraint; 4] = [
    Constraint::Percentage(40),
    Constraint::Percentage(20),
    Constraint::Percentage(20),
    Constraint::Percentage(20),
];
pub const HELP_WINDOW_CONSTRANTS: [Constraint; 3] = [
    Constraint::Percentage(10),
    Constraint::Percentage(80),
    Constraint::Percentage(10),
];
pub const INNER_HELP_VERTICAL: [Constraint; 4] = [
    Constraint::Percentage(5),
    Constraint::Percentage(45),
    Constraint::Percentage(45),
    Constraint::Percentage(5),
];
pub const INNER_HELP_HORIZONTAL: [Constraint; 3] = [
    Constraint::Percentage(5),
    Constraint::Percentage(90),
    Constraint::Percentage(5),
];

pub const STATUS_BAR_VERTICAL: [Constraint; 3] = [
    Constraint::Percentage(70),
    Constraint::Percentage(20),
    Constraint::Percentage(10),
];
