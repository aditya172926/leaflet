use ratatui::{Frame, crossterm::event::{KeyCode, KeyEvent}, layout::{Constraint, Layout}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, ListState}};

#[cfg(feature = "web3")]
use crate::structs::Feature;
use crate::{renders::render_widgets::render_paragraph::{self, paragraph_widget}, structs::{AppState, StomataState}};

impl StomataState {
    fn new() -> Self {
        let mut available_features = Vec::new();
        
        #[cfg(feature = "core")]
        available_features.push(Feature::Core);

        #[cfg(feature = "web3")]
        available_features.push(Feature::Web3);

        Self {
            state: AppState::FeatureSelection,
            selected_feature: 0,
            available_features
        }
    }

    fn render_feature_selection(&self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(5),
            Constraint::Min(10)
        ]).split(frame.area());
        
        let title = paragraph_widget("Select a feature", "Stomata ClI");
        frame.render_widget(title, chunks[0]);

        // feature list
        if self.available_features.is_empty() {
            let msg = paragraph_widget("No features available. Please install with a feature flag", "Error");
            frame.render_widget(msg, chunks[1]);
        } else {
            let items: Vec<ListItem> = self.available_features.iter().map(|feature| {
                let (name, desc) = match feature {
                    Feature::Core => ("System Monitor", "Monitor CPU, Memory and Swap, Disk and Network metrics"),
                    Feature::Web3 => ("Web3 Tools", "Access web3 tools for address validation and more...")
                };

                ListItem::new(vec![
                    Line::from(Span::styled(name, Style::default().add_modifier(Modifier::BOLD))),
                    Line::from(Span::styled(desc, Style::default().fg(Color::Gray))),
                ])
            }).collect();

            let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Available Features"))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            )
            .highlight_symbol("► ");

            let mut state = ListState::default();
            state.select(Some(self.selected_feature));
            frame.render_stateful_widget(list, chunks[1], &mut state);
        }
    }

    fn handle_feature_selection(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Up => {
                if self.selected_feature > 0 {
                    self.selected_feature -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_feature < self.available_features.len().saturating_sub(1) {
                    self.selected_feature += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(&feature) = self.available_features.get(self.selected_feature) {
                    self.state = AppState::RunningFeature(feature);
                }
            }
            KeyCode::Char('q') => {
                return false; // Exit app
            }
            _ => {}
        }
        true // Continue running
    }
}