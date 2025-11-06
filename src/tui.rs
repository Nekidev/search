use std::{sync::Arc, time::Duration};

use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{List, Paragraph},
};

use crate::state::{QueryState, State};

pub fn run(state: Arc<State>) -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let result = tui_loop(&mut terminal, &state);

    ratatui::restore();

    result
}

fn tui_loop(terminal: &mut ratatui::DefaultTerminal, state: &State) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;

        if handle_events(state)? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, state: &State) {
    if let QueryState::Searching = *state.results.read().unwrap() {
        render_searching(frame, state);
    } else {
        render_results(frame, state);
    }
}

fn render_searching(frame: &mut Frame, state: &State) {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);

    let [body_area, footer_area] = layout.areas(frame.area());

    let content = Paragraph::new(format!("Searching for '{}'...", state.query.query));
    frame.render_widget(content, body_area);

    let footer = Line::styled("Press 'q' to quit", Style::new().bg(Color::White));
    frame.render_widget(footer, footer_area);
}

fn render_results(frame: &mut Frame, state: &State) {
    let layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Fill(1),
        Constraint::Length(1),
    ]);
    let [header_area, body_area, footer_area] = layout.areas(frame.area());

    let result = state.results.read().unwrap();
    let result = result.unwrap_results();

    let paragraph = Paragraph::new(format!(
        "Found {} results in {} ms for '{}'",
        result.search_information.formatted_total_results,
        result.search_information.formatted_search_time,
        state.query.query
    ));
    frame.render_widget(paragraph, header_area);

    let items = result.items.clone();
    let list = List::new(add_gaps_to_list_items(items.into_iter().map(list_item)))
        .highlight_style(
            Style::new()
                .remove_modifier(Modifier::DIM)
                .bg(Color::LightBlue)
                .fg(Color::White),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, body_area, &mut state.list.write().unwrap());

    let footer = Line::styled("Press 'q' to quit", Style::new().bg(Color::White));
    frame.render_widget(footer, footer_area);
}

fn list_item<'a>(result: crate::google::GoogleSearchResult) -> Text<'a> {
    let mut text = Text::default();

    text.push_line(Line::from(vec![
        Span::styled(result.title, Style::new().fg(Color::Yellow).bold()),
        Span::styled(
            format!(" - {}", result.display_link),
            Style::new().add_modifier(Modifier::DIM),
        ),
    ]));
    text.push_line(Line::styled(result.snippet, Style::new().fg(Color::Gray)));

    text
}

fn add_gaps_to_list_items<'a>(
    items: impl Iterator<Item = Text<'a>>,
) -> impl Iterator<Item = Text<'a>> {
    items
        .flat_map(|item| Vec::from([Text::from(Line::from(Span::raw(""))), item]))
        .skip(1)
}

fn handle_events(state: &State) -> std::io::Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                if let KeyCode::Char('q') = key.code {
                    return Ok(true);
                } else if let KeyCode::Down = key.code {
                    let mut list = state.list.write().unwrap();

                    list.select_next();
                    list.select_next();
                } else if let KeyCode::Up = key.code {
                    let mut list = state.list.write().unwrap();

                    list.select_previous();
                    list.select_previous();
                } else if let KeyCode::PageDown = key.code {
                    let mut list = state.list.write().unwrap();

                    list.select_last();
                } else if let KeyCode::PageUp = key.code {
                    let mut list = state.list.write().unwrap();

                    list.select_first();
                } else if let KeyCode::Enter = key.code {
                    if let QueryState::Finished(Ok(results)) = &*state.results.read().unwrap() {
                        let list = state.list.read().unwrap();

                        // Remove the gaps added between items
                        let selected_index = list.selected().unwrap() / 2;
                        let selected_item = &results.items[selected_index];

                        let _ = open::that(&selected_item.link);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(false)
}
