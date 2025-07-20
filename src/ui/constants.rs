use ratatui::layout::Constraint;

pub const HEADER_ASCII: &str = r#"
  _________                        .___     .__                   .___          __        .__ 
 /   _____/ ____  __ __  ____    __| _/____ |  |   ____  __ __  __| _/        _/  |_ __ __|__|
 \_____  \ /  _ \|  |  \/    \  / __ |/ ___\|  |  /  _ \|  |  \/ __ |  ______ \   __\  |  \  |
 /        (  <_> )  |  /   |  \/ /_/ \  \___|  |_(  <_> )  |  / /_/ | /_____/  |  | |  |  /  |
/_______  /\____/|____/|___|  /\____ |\___  >____/\____/|____/\____ |          |__| |____/|__|
        \/                  \/      \/    \/                       \/
"#;

pub const MAIN_CONSTRAINTS: [Constraint; 3] = [Constraint::Length(3), Constraint::Min(0), Constraint::Length(6)];
pub const TOP_BAR_CONSTRAINTS: [Constraint; 2] = [Constraint::Percentage(95), Constraint::Percentage(5)];
pub const BODY_CONSTRAINTS: [Constraint; 2] = [Constraint::Percentage(85), Constraint::Percentage(15)];
pub const BODY_BAR_CONSTRAINTS: [Constraint; 2] = [Constraint::Ratio(1,2); 2];
