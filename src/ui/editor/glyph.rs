use std::fmt::Display;

#[derive(Debug)]
pub enum Glyph {
  Char(char),
  Cursor,
  // HtmlNode(String),
}

impl Display for Glyph {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Glyph::Char(c) => write!(f, "{}", c),
      // TODO: custom cursor
      Glyph::Cursor => write!(f, "▫️"),
      // HTMLNode => write!(f, "HTMLNode"),
    }
  }
}
