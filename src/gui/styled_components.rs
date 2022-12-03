use iced::{
    alignment,
    widget::{self, text, Button, Column, Row},
    Color,
};

pub const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};

pub fn styled_column<Message: Clone>(title: Option<&str>) -> Column<Message> {
    let mut column = widget::column![];
    if let Some(title) = title {
        column = column.push(text(title).size(50));
    }
    column.spacing(20)
}

pub fn styled_row<'a, Message: Clone>() -> Row<'a, Message> {
    widget::row![].spacing(20)
}

pub fn styled_button<Message: Clone>(label: &str) -> Button<Message> {
    widget::button(text(label).horizontal_alignment(alignment::Horizontal::Center))
        .padding(10)
        .width(iced::Length::Units(150))
}

pub fn styled_error<Message: Clone>(error: &anyhow::Error) -> Column<Message> {
    let text = text(error.to_string()).style(RED);
    widget::column![text].spacing(5)
}
