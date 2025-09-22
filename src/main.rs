use dioxus::{
  desktop::{Config, WindowBuilder},
  prelude::*,
};
use std::{error::Error, fs};
// use tree_sitter::{Parser, TreeCursor};

use crate::ui::{bar::Bar, editor::editor::Editor, home::Home, welcome::Welcome};

mod ui;

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
  dioxus::LaunchBuilder::new()
    .with_cfg(
      Config::default()
        // .with_menu(None)
        .with_disable_context_menu(true)
        .with_window(WindowBuilder::new().with_maximized(true).with_title("dvil")),
    )
    .launch(App);

  Ok(())
}

#[component]
fn App() -> Element {
  let source = "src/main.rs";
  let file = match fs::read_to_string(&source) {
    Ok(file) => file,
    Err(_) => "Failed to open file!".to_string(),
  };

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

    // Float { }

    Router::<Route> { }
  }
}
