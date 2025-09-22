use dioxus::prelude::*;

#[component]
pub fn Editor(file: String, font: String) -> Element {
  let mut text = use_signal(|| file);
  let line_count = text.read().lines().count().max(1);

  // TODO: get cursor row and column
  // let handle_click = move || {};

  rsx! {
    div {
      style: "display: flex; font-family: {font};",

      div {
        style: "
          text-align: right;
          margin: 0rem 1rem 0rem 2rem;
          color: gray;
          user-select: none;
          line-height: 1.2rem;
        ",

        for i in 1..=line_count {
          div { "{i}" }
        }
      }

      textarea {
        style: "
          flex: 1;
          white-space: pre;
          background-color: #0f1116;
          color: #ffffff;
          border: none;
          resize: none;
          outline: none;
          overflow: hidden;
          padding: 0px;
          // margin: 0px;
          caret-color: red;
          font-family: {font};
        ",
        value: "{text}",

        oninput: move |e| text.set(e.value()),
        // onclick: todo!(),
      }
    }
  }
}

#[component]
pub fn Cursor(row: u64, col: u64) -> Element {
  rsx! {}
}
