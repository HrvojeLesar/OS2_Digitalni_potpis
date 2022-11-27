use iced::{
    alignment,
    widget::{self, text, Button, Column, Row},
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
