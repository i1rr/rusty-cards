use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders,Paragraph},
    Frame,
};
use ratatui::layout::Alignment;


use crate::app::{App, CurrentScreen};
use crate::sqlite::FlashCard;

pub fn ui(frame: &mut Frame, app: &mut App, fc: &mut FlashCard) {

    let layout_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(70)
        ]).split(frame.size());

    let layout_word_and_shortcuts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(70),
            Constraint::Percentage(30)
        ]).split(layout_main[0]);

    frame.render_widget(
        Paragraph::new("This word means nothing :)")
            .block(Block::new().borders(Borders::ALL).title("Examples and definition")),
        layout_main[1]);
    frame.render_widget(
        Paragraph::new("hello rusty").alignment(Alignment::Center)
            .block(Block::new().borders(Borders::ALL)
                .title("Ready to Flip? Press ENTER")
                .title_alignment(Alignment::Center)),
        layout_word_and_shortcuts[0]);
    frame.render_widget(
        Paragraph::new(
            "'q' - close the app\n\
        'a' - add a new word")
            .block(Block::new().borders(Borders::ALL).title("Key Bindings")),
        layout_word_and_shortcuts[1]);
}