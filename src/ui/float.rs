use dioxus::prelude::*;

#[component]
pub fn Float() -> Element {
  let mut active = use_signal(|| true);
  // let dialog_ref = use_reactive();

  rsx! {
    dialog {
      style: "
        position: fixed;
        top: 10vw;
        left: 0vh;
        width: 500px;
        height: 500px;
        background-color: #0f1116;
        border: 2px solid black;
      ",
      open: *active.read(),

      p { "Test dialog element!" }

      button {
        autofocus: true,

        onclick: move |_| {
          let current = *active.read();
          active.set(!current);
          println!("{current}");
        },

        "Success",
      }
    }
  }
}
