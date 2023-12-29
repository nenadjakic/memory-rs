use std::char;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    ascii_constants::{ASCII_A, ASCII_EMPTY},
    main,
};

pub fn render(frame: &mut Frame) {
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
    inner_area.x += 4;
    inner_area.width -= 8;
    inner_area.y += 2;
    inner_area.height -= 4;

    frame.render_widget(main_block, area);

    render_game_area(frame, inner_area);
}

pub fn render_game_area(f: &mut Frame, r: Rect) {
    let layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        //.constraints(vec![Constraint::Length(r.height / 4); 4])
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(r);

    for (row_index, row) in layout_vertical.iter().enumerate() {
        let layout_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            //.constraints(vec![Constraint::Length(row.width / 4); 4])
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(*row);

        for (slot_index, slot) in layout_horizontal.iter().enumerate() {
            let mut box_color = Color::LightYellow;

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(ratatui::style::Style::default().fg(box_color));

            let text_case_widget = Paragraph::new(render_ascii_char('a'))
                .block(block.clone())
                .alignment(Alignment::Center)
                .style(ratatui::style::Style::default().fg(Color::LightRed));
            f.render_widget(text_case_widget, *slot);
            f.render_widget(block, *slot);
        }
    }
}

fn render_ascii_char(c: char) -> String {
    let mut result: String = String::new();
    let ascii = match c {
        'a' => ASCII_A,
        _ => ASCII_EMPTY,
    };
    for &x in ascii.iter() {
        result += x;
        result += "\n";
    }
    return result;
}
