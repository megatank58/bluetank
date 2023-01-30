use btleplug::api::Peripheral;
use crossterm::event::{self, Event, KeyCode};
use std::{
    error::Error,
    io::Stdout,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

use crate::manager::BlueManager;

pub struct App {
    manager: BlueManager,
    items: Vec<String>,
}

impl App {
    pub fn new() -> App {
        Self {
            manager: BlueManager::new(),
            items: vec![],
        }
    }

    pub async fn render(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        tick_rate: Duration,
    ) -> Result<(), Box<dyn Error>> {
        let mut last_tick = Instant::now();
        loop {
            println!("{}", self.manager.list().await?.len());

            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        // KeyCode::Enter => self.connect(),
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                // app.on_tick();
                last_tick = Instant::now();
            };
        }
    }

    pub fn ui(&mut self, f: &mut Frame<CrosstermBackend<Stdout>>) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| {
                ListItem::new(String::from(i))
                    .style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        f.render_widget(items, chunks[0]);
    }
}
