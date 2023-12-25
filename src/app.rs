use std::error::Error;
use crate::sqlite::FlashCard;
use rand::seq::SliceRandom;

pub enum CurrentScreen {
    Main,
    Reveal,
    Adding,
    Editing,
    Exiting,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub list: Vec<FlashCard>,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            list: App::get_shuffled_flashcards().unwrap()
        }
    }

    pub fn get_shuffled_flashcards() -> Result<Vec<FlashCard>, Box<dyn Error>> {
        let mut cards = FlashCard::fetch_next_ten()?;

        let mut rng = rand::thread_rng();
        cards.shuffle(&mut rng);

        Ok(cards)
    }

    pub fn pop_flashcard(&mut self) -> Option<FlashCard> {
        self.list.pop()
    }

    pub fn print_result(&self) {
        // TODO: retrieve stats from db and print them out
    }
}