use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use crate::app::{App, AppMode};

pub fn draw(f: &mut Frame, app: &App) {
    match &app.mode {
        AppMode::Menu { selected } => draw_menu(f, *selected),
        AppMode::Running(anim) => anim.render(f),
    }
}

fn draw_menu(f: &mut Frame, selected: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(11),
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_header(f, chunks[0]);
    draw_description(f, chunks[1]);
    draw_menu_list(f, chunks[2], selected);
    draw_footer(f, chunks[3]);
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

fn draw_description(f: &mut Frame, area: ratatui::layout::Rect) {
    let text = Paragraph::new("Terminal animation showcase. Select an animation to begin.")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(text, area);
}

fn draw_menu_list(f: &mut Frame, area: ratatui::layout::Rect, selected: usize) {
    let items = vec![
        "[1] Starfield (working)",
        "[2] Matrix (working)",
        "[3] Wave (working)",
        "[4] Snake (working)",
        "[5] Fire (working)",
        "[6] Rain (working)",
        "[7] Swarm (working)",
        "[8] Circuit (working)",
        "[9] Void (working)",
    ];

    let menu_items: Vec<ListItem> = items
        .into_iter()
        .map(|item| ListItem::new(item))
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected));

    let menu = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title(" Animations "))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(menu, area, &mut state);
}

fn draw_footer(f: &mut Frame, area: ratatui::layout::Rect) {
    let tips = Line::from(vec![
        Span::styled("1-9", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("/"),
        Span::styled("Enter", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" launch  "),
        Span::styled("↑↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" navigate  "),
        Span::styled("q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" quit"),
    ]);

    let footer = Paragraph::new(tips)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(Color::Gray));

    f.render_widget(footer, area);
}