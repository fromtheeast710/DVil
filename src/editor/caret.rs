use crate::editor::motion::{Action, Motion};
use crop::Rope;

#[derive(Debug, Default)]
pub struct Caret {
  pub x: usize,
  pub y: usize,
}

impl Caret {
  pub fn new() -> Self {
    // TODO: remember previous position
    Self { x: 1, y: 1 }
  }

  pub fn update(&mut self, content: &Rope, line_count: usize, act: &Action) -> Self {
    match act {
      Action::Delete => {
        // TODO: handle newline
        self.x = self.x.saturating_sub(1);
      }
      Action::Insert(c) => {
        if c == "\n" {
          self.x = 1;
          self.y = self.y.saturating_add(1);
        } else {
          self.x = self.x.saturating_add(1);
        }
      }
      Action::Move(m) => match m {
        Motion::Left(n) => self.x = self.x.saturating_sub(*n),
        // TODO: subtract next/prev line len by 1
        Motion::Down(n) => {
          self.x = self.x.min(content.line(self.y.saturating_add(1)).to_string().len());
          self.y = (self.y.saturating_add(*n)).min(line_count - 1);
        }
        Motion::Up(n) => {
          self.x = self.x.min(content.line(self.y.saturating_sub(1)).to_string().len());
          self.y = (self.y.saturating_sub(*n)).min(line_count - 1);
        }
        Motion::Right(n) => {
          self.x = self
            .x
            .saturating_add(*n)
            .min(content.line(self.y).to_string().len().saturating_sub(1))
        }
      },
    }

    Self {
      x: self.x,
      y: self.y,
    }
  }
}
