use crate::editor::{
  caret::Caret,
  config::EditorConfig,
  file::File,
  motion::{Action, Motion},
};
use crop::{Rope, RopeBuilder};
use iced::{
  Color, Element, Font, Length, Pixels, Point, Rectangle, Renderer, Size, Subscription, Task,
  Theme,
  event::{self, Status},
  font::{Family, Stretch, Style, Weight},
  keyboard::{Event::KeyPressed, Key, key::Named},
  mouse,
  widget::{
    canvas::{self, Frame, Program, Text},
    text::LineHeight,
  },
};
// use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct Editor {
  file: File,
  content: Rope,
  config: EditorConfig,
  caret: Caret,
}

impl Program<Action> for Editor {
  type State = ();

  fn draw(
    &self,
    _state: &Self::State,
    renderer: &Renderer,
    _theme: &Theme,
    bounds: Rectangle,
    _cursor: mouse::Cursor,
  ) -> Vec<canvas::Geometry<Renderer>> {
    // TODO: load custom monospace font
    // const FIRACODE_NERD_FONT: Font = Font::with_name("FiraCodeMono");
    // const FIRACODE_NERD_FONT_BYTES: &[u8] = include_bytes!("../asset/font/FiraCodeNerd.ttf");
    // generate_icon_functions!("font.ttf", iced_aw_font, ICED_AW_FONT);

    let mut frame = Frame::new(renderer, bounds.size());

    let cfg = &self.config;

    let char_height = 16.5;
    let char_width = 9.85;
    let line_visible = (bounds.height / char_height).floor();
    let num_width = cfg.num_pad.left
      + cfg.num_pad.right
      + self.file.line_count.to_string().chars().count() as f32 * char_width;

    // println!("{line_visible}");

    // TODO: use hex instead of rgba8
    macro_rules! fill_text {
      ($pos_x:expr,$pos_y:expr,$content:expr) => {
        frame.fill_text(Text {
          content: $content.to_string(),
          position: Point::new($pos_x, $pos_y),
          font: Font {
            family: Family::Monospace,
            weight: Weight::Light,
            stretch: Stretch::Normal,
            style: Style::Normal,
          },
          line_height: LineHeight::Absolute(Pixels(char_height)),
          color: Color::from_rgba8(0, 0, 0, 100.),
          ..Default::default()
        })
      };
    }

    frame.fill_rectangle(
      Point::ORIGIN,
      bounds.size(),
      Color::from_rgb8(200, 200, 200),
    );

    frame.fill_rectangle(
      Point::new(num_width + char_width * 80., 0.0),
      Size::new(11.0, bounds.height),
      Color::from_rgba8(2, 240, 240, 99.2),
    );

    fill_text!(
      bounds.width - char_width * 3.,
      bounds.height - char_height * 2.,
      self.caret.x + 1
    );
    fill_text!(
      bounds.width - char_width * 3.,
      bounds.height - char_height,
      self.caret.y + 1
    );

    for (i, line) in self.content.lines().enumerate() {
      let row = i as f32 * char_height;
      let offset = self.caret.y.saturating_sub(line_visible as usize) as f32 * char_height;
      let rowoff = row - offset;

      if i == self.caret.y {
        for (j, char) in line.chars().enumerate() {
          let col = num_width + j as f32 * char_width;

          if j == self.caret.x {
            frame.fill_rectangle(
              Point::new(col, rowoff),
              Size::new(char_width, char_height),
              Color::from_rgb8(23, 255, 255),
            );

            fill_text!(col, rowoff, char);
          } else {
            fill_text!(col, rowoff, char);
          }
        }

        fill_text!(cfg.num_pad.left, rowoff, (i + 1));
      } else {
        fill_text!(num_width, rowoff, line);
        fill_text!(cfg.num_pad.left, rowoff, (i + 1));
      }
    }

    vec![frame.into_geometry()]
  }

  fn update(
    &self,
    _state: &mut Self::State,
    _event: &iced::Event,
    _bounds: Rectangle,
    _cursor: mouse::Cursor,
  ) -> std::option::Option<iced::widget::Action<Action>> {
    None
  }

  fn mouse_interaction(
    &self,
    _state: &Self::State,
    _bounds: iced::Rectangle,
    _cursor: mouse::Cursor,
  ) -> mouse::Interaction {
    mouse::Interaction::None
  }
}

impl Editor {
  pub fn new() -> (Self, Task<self::Action>) {
    let file = File::new("./src/editor/editor.rs");
    let caret = Caret::new();

    let mut content = RopeBuilder::new();
    content.append(&file.content);

    (
      Self {
        file,
        caret: Caret {
          x: caret.x - 1,
          y: caret.y - 1,
        },
        content: content.build(),
        config: EditorConfig {
          ..Default::default()
        },
      },
      Task::none(),
    )
  }

  pub fn update(&mut self, act: Action) -> Task<Action> {
    let caret_pos = if self.caret.y == 0 {
      self.caret.x
    } else {
      self.content.byte_of_line(self.caret.y) + self.caret.x
    };

    self.caret.update(&self.content, self.file.line_count, &act);

    // println!("{}, {}, {}", self.caret.x, self.caret.y, caret_pos);

    match act {
      Action::Delete => {
        self.content.delete(caret_pos.saturating_sub(1)..caret_pos);
      }
      Action::Insert(c) => {
        if c == "\n" {
          self.content.insert(caret_pos, c)
        } else {
          self.content.insert(caret_pos, c);
        }
      }
      Action::Move(_) => (),
    }

    Task::none()
  }

  pub fn view(&'_ self) -> Element<'_, Action> {
    canvas::Canvas::new(self).width(Length::Fill).height(Length::Fill).into()
  }

  pub fn subscribe(&self) -> Subscription<Action> {
    event::listen_with(|event, status, _id| match (event, status) {
      // TODO: key combos, tab, shift, ctrl, etc
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Character(c),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Insert(c.to_string())),
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Named(Named::Enter),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Insert("\n".to_string())),
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Named(Named::Space),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Insert(" ".to_string())),
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Named(Named::Tab),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Insert("  ".to_string())),
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Named(Named::Backspace),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Delete),
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Named(Named::ArrowLeft),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Move(Motion::Left(1))),
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Named(Named::ArrowRight),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Move(Motion::Right(1))),
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Named(Named::ArrowUp),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Move(Motion::Up(1))),
      (
        iced::Event::Keyboard(KeyPressed {
          key: Key::Named(Named::ArrowDown),
          ..
        }),
        Status::Ignored,
      ) => Some(Action::Move(Motion::Down(1))),
      _ => None,
    })
  }
}

// TODO: blank intro page
// impl Default for Editor {
//   fn default() -> Self {
//     let file = File::new("./src/editor/editor.rs");
//     let caret = Caret::new();
//
//     let mut content = RopeBuilder::new();
//     content.append(&file.content);
//
//     Self {
//       file,
//       caret,
//       content: content.build(),
//       config: EditorConfig {
//         ..Default::default()
//       },
//     }
//   }
// }
