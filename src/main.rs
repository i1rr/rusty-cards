use std::error::Error;
use ratatui::backend::{Backend};

mod ui;
mod app;
mod sqlite;
mod controller;

fn main() -> Result<(), Box<dyn Error>> {
  controller::start()
}
