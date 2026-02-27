use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders, BorderType, Padding, Paragraph};
use ratatui::Terminal;
use ratatui::crossterm::event::{Event, KeyEvent, KeyCode, KeyModifiers};
use ratatui::prelude::*;

use std::io;
use tui_textarea::{Input, Key, TextArea};
use ratatui::style::{Style, Color, Modifier};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    ratatui::crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_max_histories(1000);
    // Make the current line just be normal (could also make it bold, italic, or bg color or something here)
    textarea.set_cursor_line_style(Style::default());

    textarea.set_block(
        Block::default()
            .style(Style::default().bg(Color::from_u32(0x00222222)).fg(Color::White))
            .borders(Borders::LEFT)
            .border_type(BorderType::QuadrantOutside)
            .border_style(Style::default().fg(Color::from_u32(0x008888ff)))
            .padding(Padding::symmetric(2, 1))
    );

    let mut statusarea = Paragraph::new("Status")
        .block(Block::new()
            .style(Style::default().bg(Color::from_u32(0x00141414)).fg(Color::White).add_modifier(Modifier::BOLD))
            .borders(Borders::NONE)
            .padding(Padding::symmetric(2, 1))
        );

    loop {
        // Show line numbers if there is more than 1 line
        textarea.remove_line_number();
        if textarea.lines().len() > 1 {
            textarea.set_line_number_style(Style::default().fg(Color::Gray).add_modifier(Modifier::DIM));
        }
        term.draw(|f| {

            let outer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(100),
                    Constraint::Min(40),
                ])
                .split(f.area());

            f.render_widget(&textarea, outer_layout[0]);
            f.render_widget(&statusarea, outer_layout[1]);
        })?;
        let inp = ratatui::crossterm::event::read()?;
        let inp_r: tui_textarea::Input = inp.clone().into();
        match inp {
            ratatui::crossterm::event::Event::Key(KeyEvent { code: KeyCode::Esc, ..}) => break,
            // ALT-ENTER always submits
            Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::ALT, ..}) => {
                break
            }
            Event::Key(KeyEvent { code: KeyCode::Enter, ..}) => {
                if textarea.lines().len() == 1 {
                    // ENTER on single line input submits
                    break
                } else {
                    textarea.input(inp_r);
                }
            },
            // CTRL-J is translated to ENTER, use to press ENTER without submit
            Event::Key(KeyEvent { code: KeyCode::Char('j'), modifiers: KeyModifiers::CONTROL, ..}) => {
                textarea.input(Input { key: Key::Enter, ctrl: false, alt: false, shift: false });
            }
            _ => {
                textarea.input(inp_r);
            }
        }
    }

    disable_raw_mode()?;
    ratatui::crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Lines: {:?}", textarea.lines());
    Ok(())
}
