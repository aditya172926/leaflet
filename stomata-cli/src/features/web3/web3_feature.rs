use std::{
    io::Stdout,
    iter::once,
    process::exit,
    time::{Duration, Instant},
};

use clap::Parser;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Tabs},
};

use crate::{
    features::web3::cli::{KeySubCommands, Web3Cli, Web3Tool},
    renders::{
        render_widgets::render_paragraph::paragraph_widget,
        web3_displays::{
            address_validation::validate_address,
            key_encryption::{decrypt_key, encrypt_key},
        },
    },
    structs::Cli,
};

pub enum Web3Page {
    AddressValidation,
}

impl Web3Page {
    pub fn titles() -> Vec<&'static str> {
        vec!["Address Validation"]
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Web3Page::AddressValidation,
            _ => Web3Page::AddressValidation,
        }
    }
}

pub struct Web3UIState;

pub struct Web3State {
    pub render: bool,
    pub current_page: Web3Page,
    pub tab_index: usize,
    pub ui_state: Option<Web3UIState>,
}

impl Web3State {
    pub fn new() -> Self {
        Self {
            render: true,
            current_page: Web3Page::AddressValidation,
            tab_index: 0,
            ui_state: None,
        }
    }

    // go to the next tab
    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % Web3Page::titles().len();
        self.current_page = Web3Page::from_index(self.tab_index);
    }

    // go to the previous tab
    pub fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = Web3Page::titles().len() - 1;
        }
        self.current_page = Web3Page::from_index(self.tab_index);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let chunks =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

        // render tabs
        self.render_tabs(frame, chunks[0]);

        match &self.current_page {
            Web3Page::AddressValidation => {
                let para = paragraph_widget(
                    "Hi! We are adding more interactive features to Stomata Web3",
                    "About",
                );
                frame.render_widget(para, chunks[1]);
            }
        }
    }

    // render tabs
    pub fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let titles: Vec<Line> = Web3Page::titles().iter().map(|t| Line::from(*t)).collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Stomata"))
            .select(self.tab_index)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_widget(tabs, area);
    }

    // handle events
    pub fn handle_events(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        if key.kind == KeyEventKind::Press {
            self.process_global_events(key);
        }
        Ok(())
    }

    fn process_global_events(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.render = false;
            }
            KeyCode::Right | KeyCode::Tab => {
                self.next_tab();
            }
            KeyCode::Left => {
                self.previous_tab();
            }
            KeyCode::Char('1') => {
                self.tab_index = 0;
                self.current_page = Web3Page::AddressValidation;
            }
            _ => {}
        }
    }
}

pub fn run(
    cli: &Cli,
    terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>,
) -> anyhow::Result<bool> {
    let mut web3_state = Web3State::new();

    match terminal {
        Some(terminal) => {
            // get the refresh interval from the cli arg. Default 1000 ms
            let refresh_interval = Duration::from_millis(cli.interval);
            let mut last_tick = Instant::now();

            while web3_state.render {
                let timeout = refresh_interval
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(Duration::from_secs(0));

                // poll for inputs only until timeout
                if event::poll(timeout)? {
                    if let Event::Key(key) = event::read()? {
                        // handle events
                        web3_state.handle_events(key)?;
                        // redraw immediately after an event
                        terminal.draw(|frame| web3_state.render(frame))?;
                    }
                }

                if last_tick.elapsed() >= refresh_interval {
                    // draw
                    terminal.draw(|frame| web3_state.render(frame))?;
                    last_tick = Instant::now();
                }
            }
            Ok(web3_state.render)
        }
        None => {
            let web3_cli =
                Web3Cli::try_parse_from(once("web3".to_string()).chain(cli.args.iter().cloned()));
            match web3_cli {
                Ok(cli) => {
                    match cli.tool {
                        Web3Tool::AddressValidator { address } => validate_address(&address),
                        Web3Tool::Key(key_cmd) => match key_cmd {
                            KeySubCommands::Encrypt { name } => encrypt_key(name),
                            KeySubCommands::Decrypt { name, format } => decrypt_key(name, format),
                        },
                    };
                }
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            };
            Ok(false)
        }
    }
}
