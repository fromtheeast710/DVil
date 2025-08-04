use dioxus::prelude::*;
use std::{error::Error, fs};
use tree_sitter::{Parser, TreeCursor};

use crate::ui::{bar::Bar, editor::Editor, float::Float, home::Home, welcome::Welcome};

mod ui {
  pub mod bar;
  pub mod cursor;
  pub mod editor;
  pub mod float;
  pub mod home;
  pub mod welcome;
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
  #[layout(Bar)]
  #[route("/")]
  App {},
  #[route("/home")]
  Home {},
  #[route("/welcome")]
  Welcome {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const FONT_FIRA_CODE: Asset = asset!("/assets/fonts/FiraCodeNerdFontMonoRegular.ttf");

fn main() -> Result<(), Box<dyn Error>> {
  dioxus::launch(App);

  Ok(())
}

#[component]
fn App() -> Element {
  let mut parser = Parser::new();
  let language = tree_sitter_rust::LANGUAGE;
  parser.set_language(&language.into()).expect("Error loading rust parser");

  let source = "src/main.rs";
  let file = match fs::read_to_string(&source) {
    Ok(file) => file,
    Err(_) => "Failed to open file!".to_string(),
  };
  let _tree = parser.parse(&file, None).unwrap();

  // HACK: load font using only css?
  let fira = format!(
    r#"
    @font-face {{
      font-family: "nerdfont";
      src: url("{FONT_FIRA_CODE}") format("truetype");
      font-weight: normal;
      font-style: normal;
    }}
    "#,
  );

  rsx! {
     document::Link { rel: "icon", href: FAVICON }
     document::Link { rel: "stylesheet", href: MAIN_CSS }

     style { "{fira}" }

     Editor { file: file, font: FONT_FIRA_CODE }

     Router::<Route> {}
  }
}
