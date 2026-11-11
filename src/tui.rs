use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::backend;

struct Message {
    user: String,
    output: Vec<String>,
}

#[derive(Default)]
struct App {
    input: String,
    messages: Vec<Message>,
    should_quit: bool,
}

impl App {
    fn submit(&mut self) {
        if self.input.is_empty() {
            return;
        }
        let user = std::mem::take(&mut self.input);
        let output = backend::handle_input(&user);
        self.messages.push(Message { user, output });
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }

            match (key.code, key.modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.should_quit = true,
                (KeyCode::Enter, _) => self.submit(),
                (KeyCode::Backspace, _) => {
                    self.input.pop();
                }
                (KeyCode::Char(ch), modifiers) if !modifiers.contains(KeyModifiers::CONTROL) => {
                    self.input.push(ch);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [history_area, input_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).areas(frame.area());
        let lines: Vec<Line> = self
            .messages
            .iter()
            .flat_map(|message| {
                std::iter::once(Line::from(vec![
                    Span::styled("You: ", Style::default().fg(Color::Cyan)),
                    Span::raw(message.user.clone()),
                ]))
                .chain(message.output.iter().map(|output| {
                    Line::from(vec![
                        Span::styled("Assistant: ", Style::default().fg(Color::Green)),
                        Span::raw(output.clone()),
                    ])
                }))
                .chain(std::iter::once(Line::raw("")))
            })
            .collect();
        frame.render_widget(
            Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title("pi harness"))
                .wrap(Wrap { trim: false }),
            history_area,
        );
        frame.render_widget(
            Paragraph::new(self.input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input")),
            input_area,
        );
        frame.set_cursor_position((input_area.x + 1 + self.input.len() as u16, input_area.y + 1));
    }
}

pub fn run() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
