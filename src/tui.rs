// // src/tui.rs
// use std::io::{self, stdout};
// use tui::{
//     backend::CrosstermBackend,
//     Terminal,
//     widgets::{Block, Borders},
//     layout::{Layout, Constraint, Direction},
//     text::{Span},
//     style::{Color, Modifier, Style},
// };

// pub fn run_tui() -> Result<(), io::Error> {
//     let stdout = stdout();
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     terminal.draw(|frame| {
//         let size = frame.size();
        
//         let title_style = Style::default()
//             .fg(Color::Cyan)        // Set foreground text color to Cyan
//             .bg(Color::Black)       // Set background color to Black
//             .add_modifier(Modifier::BOLD);  // Make the text bold

//         let block = Block::default()
//             .title(Span::styled("API Tester", title_style)) // Apply the styling here
//             .borders(Borders::ALL);

//         frame.render_widget(block, size);
//     })?;

//     Ok(())
// }
