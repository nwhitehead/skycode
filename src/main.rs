use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders, BorderType, Padding, Paragraph};
use ratatui::Terminal;
use ratatui::crossterm::event::{Event, KeyEvent, MouseEventKind, MouseEvent, KeyCode, KeyModifiers};
use ratatui::crossterm::event::MouseEventKind::{ ScrollDown, ScrollUp };
use ratatui::prelude::*;

use std::io;
use tui_textarea::{Input, Key, TextArea};
use ratatui::style::{Style, Color, Modifier};

fn fresh_input_textarea() -> TextArea<'static> {
    let mut textarea = TextArea::default();
    textarea.set_max_histories(1000);
    textarea.set_cursor_line_style(Style::default());
    textarea.set_block(
        Block::default()
            .style(Style::default().bg(Color::from_u32(0x00222222)).fg(Color::White))
            .borders(Borders::LEFT)
            .border_type(BorderType::QuadrantOutside)
            .border_style(Style::default().fg(Color::from_u32(0x008888ff)))
            .padding(Padding::symmetric(2, 1))
    );
    textarea
}

fn submit(textarea: &mut TextArea, output: &mut Vec<String>) {
    let v = textarea.lines().join("\n");
    output.push(v);
    *textarea = fresh_input_textarea();
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut output = vec![String::from("this is some output")];
    for i in 0..50 {
        let mut line = vec![];
        for j in 0..100 {
            line.push(format!("{} ", j * (i + 17)));
        }
        output.push(line.join(" "));
    }

    enable_raw_mode()?;
    ratatui::crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = fresh_input_textarea();
    let mut output_scroll: i32 = 0;

    loop {
        // Show line numbers if there is more than 1 line
        textarea.remove_line_number();
        if textarea.lines().len() > 1 {
            textarea.set_line_number_style(Style::default().fg(Color::Gray).add_modifier(Modifier::DIM));
        }
        let mut output_area = Paragraph::new(output.join("\n")).scroll((output_scroll as u16, 0));

        let mut status_area = Paragraph::new("Status")
            .block(Block::new()
                .style(Style::default().bg(Color::from_u32(0x00141414)).fg(Color::White).add_modifier(Modifier::BOLD))
                .borders(Borders::NONE)
                .padding(Padding::symmetric(2, 1))
            );

        let mut output_rect = Rect::default();
        term.draw(|f| {

            let outer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(100),
                    Constraint::Min(40),
                ])
                .split(f.area());
            let input_vsize = textarea.lines().len() + 2;
            let input_vsize = input_vsize.clamp(3, 10) as u16;
            let left_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Length(input_vsize),
                ])
                .split(outer_layout[0]);

            f.render_widget(&textarea, left_layout[1]);
            f.render_widget(&output_area, left_layout[0]);
            f.render_widget(&status_area, outer_layout[1]);
            output_rect = left_layout[0].clone();
        })?;

        let inp = ratatui::crossterm::event::read()?;
        let inp_r: tui_textarea::Input = inp.clone().into();
        match inp {
            Event::Mouse(MouseEvent { kind: ScrollDown, column, row, modifiers }) => {
                if output_rect.contains(Position { x: column, y: row }) {
                    output_scroll += 1;
                }
            }
            Event::Mouse(MouseEvent { kind: ScrollUp, column, row, modifiers }) => {
                if output_rect.contains(Position { x: column, y: row }) {
                    output_scroll -= 1;
                }
            }
            ratatui::crossterm::event::Event::Key(KeyEvent { code: KeyCode::Esc, ..}) => break,
            // ALT-ENTER always submits
            Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::ALT, ..}) => {
                submit(&mut textarea, &mut output)
            }
            Event::Key(KeyEvent { code: KeyCode::Enter, ..}) => {
                if textarea.lines().len() == 1 {
                    // ENTER on single line input submits
                    submit(&mut textarea, &mut output)
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
        // Clamp scroll in case things have changed in layout etc.
        // Max scroll cannot be less than 0
        let max_scroll = i32::max(0, output_area.line_count(output_rect.width) as i32 - output_rect.height as i32);
        if output_scroll < 0 {
            output_scroll = 0;
        }
        if output_scroll > max_scroll {
            output_scroll = max_scroll;
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
