use dioxus::prelude::*;
use std::format;

#[component]
pub fn Editor(file: String, font: String) -> Element {
  // TODO:
  // + relative and reactive number line
  // + cursor row-column highlight
  // + syntax highlight
  rsx! {
    div { style: "display: flex",
      div { style: "padding-top: 1px",
        for i in 1..file.lines().count() {
          div { {format!("{i}")} }
        }
      }

      textarea { {file} }
    }
  }
}
