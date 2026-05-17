use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crate::app::{App, AppMode, InputMode, MenuState};

pub fn draw(f: &mut Frame, app: &App) {
    match &app.mode {
        AppMode::Menu(state) => draw_menu(f, state),
        AppMode::Running(anim) => anim.render(f),
    }
}

fn draw_menu(f: &mut Frame, state: &MenuState) {
    let has_search = state.input_mode == InputMode::Search;
    let has_number = state.input_mode == InputMode::Number;
    
    let mut constraints = vec![
        Constraint::Length(3),
    ];
    
    if has_search {
        constraints.push(Constraint::Length(3));
    }
    
    constraints.push(Constraint::Length(12));
    
    if has_number {
        constraints.push(Constraint::Length(2));
    }
    
    constraints.push(Constraint::Length(3));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(f.size());

    let mut chunk_idx = 0;
    draw_header(f, chunks[chunk_idx]);
    chunk_idx += 1;

    if has_search {
        draw_search_bar(f, chunks[chunk_idx], &state.search_query);
        chunk_idx += 1;
    }

    draw_menu_list(f, chunks[chunk_idx], state);
    chunk_idx += 1;

    if has_number {
        draw_number_buffer(f, chunks[chunk_idx], &state.number_buffer);
        chunk_idx += 1;
    }

    draw_footer(f, chunks[chunk_idx], &state.input_mode);
}

fn draw_header(f: &mut Frame, area: ratatui::layout::Rect) {
    let title = Paragraph::new(Span::styled(
        " ZOR ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    )
    .style(Style::default().fg(Color::White))
    .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(title, area);
}

fn draw_search_bar(f: &mut Frame, area: ratatui::layout::Rect, query: &str) {
    let text = Paragraph::new(Span::styled(
        format!("Search: {}", query),
        Style::default().fg(Color::Yellow),
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Search ")
            .border_style(Style::default().fg(Color::Yellow)),
    )
    .alignment(ratatui::layout::Alignment::Left);

    f.render_widget(text, area);
}

fn draw_number_buffer(f: &mut Frame, area: ratatui::layout::Rect, buffer: &str) {
    let text = Paragraph::new(Span::styled(
        format!("Select #: {}", buffer),
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    ))
    .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(text, area);
}

fn draw_menu_list(f: &mut Frame, area: ratatui::layout::Rect, state: &MenuState) {
    let menu_items: Vec<ListItem> = state.filtered_indices.iter()
        .map(|&idx| {
            let label = format!("[{}] {}", idx + 1, state.items[idx]);
            ListItem::new(label)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected_index));

    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title(" Animations "))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(menu, area, &mut list_state);
}

fn draw_footer(f: &mut Frame, area: ratatui::layout::Rect, input_mode: &InputMode) {
    let tips = match input_mode {
        InputMode::Search => Line::from(vec![
            Span::styled("Esc", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" exit  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" launch"),
        ]),
        InputMode::Number => Line::from(vec![
            Span::styled("Esc", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" cancel  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" launch"),
        ]),
        InputMode::Normal => Line::from(vec![
            Span::styled("1-10", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("/"),
            Span::styled("Enter", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" launch  "),
            Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" navigate  "),
            Span::styled("/", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" search  "),
            Span::styled("q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" quit"),
        ]),
    };

    let footer = Paragraph::new(tips)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}