use std::vec;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{game_state::GameState, widgets::card::CardWidgetState};

pub fn render(frame: &mut Frame, state: &mut GameState) {
    let area = Rect::new(
        frame.size().x + 4,
        frame.size().y + 2,
        frame.size().width - 8,
        frame.size().height - 4,
    );

    let main_block = Block::default()
        .title(" Memory game ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(ratatui::style::Style::default().fg(Color::LightYellow));
    let mut inner_area = main_block.inner(area);
    inner_area.x += 2;
    inner_area.width -= 4;
    inner_area.y += 1;
    inner_area.height -= 2;

    frame.render_widget(main_block, area);

    let layout_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(inner_area);

    let board_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Plain)
        .border_style(ratatui::style::Style::default().fg(Color::LightYellow));

    for (column_index, column) in layout_horizontal.iter().enumerate() {
        frame.render_widget(board_block.clone(), *column);

        let mut inner_column = *column;
        inner_column.x += 2;
        inner_column.width -= 4;
        inner_column.y += 1;
        inner_column.height -= 2;

        if column_index == 0 {
            render_game_area(frame, inner_column, state);
        } else {
            render_manual_area(frame, inner_column, state);
        }
    }
}

pub fn render_game_area(f: &mut Frame, r: Rect, state: &mut GameState) {
    let number_of_rows_columns = state.mode.get_number_of_rows_and_columns();

    let mut i = 0;
    let mut constraints_vertical: Vec<Constraint> = Vec::new();
    let p: u16 = (100 / number_of_rows_columns.0).into();
    while i < number_of_rows_columns.0 {
        constraints_vertical.push(Constraint::Percentage(p));
        i += 1;
    }

    i = 0;
    let mut constraints_horizontal: Vec<Constraint> = Vec::new();
    let p: u16 = (100 / number_of_rows_columns.1).into();
    while i < number_of_rows_columns.1 {
        constraints_horizontal.push(Constraint::Percentage(p));
        i += 1;
    }

    let layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints_vertical)
        .split(r);

    for (row_index, row) in layout_vertical.iter().enumerate() {
        let layout_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints_horizontal.clone())
            .split(*row);

        for (slot_index, slot) in layout_horizontal.iter().enumerate() {
            let card = state
                .board
                .get(&(
                    row_index.try_into().unwrap(),
                    slot_index.try_into().unwrap(),
                ))
                .unwrap();
            f.render_stateful_widget(*card, *slot, &mut state.selected_symbol);
        }
    }
}

fn render_manual_area(f: &mut Frame, r: Rect, state: &mut GameState) {
    let mut help_text = vec![
        Line::from(Span::raw("Movement: ← ↓ ↑ →")),
        Line::from(Span::raw("Claim a card: ENTER / SPACE")),
        Line::from(Span::raw(
            "After two cards are fliped, to continue: ENTER / SPACE",
        )),
        Line::from(Span::raw("Quit: q")),
    ];

    if state.finished {
        help_text.append(&mut vec![
            Line::from(Span::default()),
            Line::from(Span::styled("You finish !!!", Style::default().add_modifier(Modifier::SLOW_BLINK))),
        ]);
    }

    let text_widget = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .style(ratatui::style::Style::default().fg(Color::LightRed));

    f.render_widget(text_widget, centered_rect(r, 100, 20))
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
