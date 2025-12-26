use crate::editor::editor::Editor;
use iced::{Result, Theme, application};

mod app;
mod editor;
mod layout;

fn main() -> Result {
  application(Editor::new, Editor::update, Editor::view)
    .title("dvil")
    .theme(Theme::Dark)
    .font(include_bytes!("../asset/font/FiraCodeNerd.ttf").as_slice())
    .subscription(Editor::subscribe)
    .antialiasing(true)
    .centered()
    .run()
}
