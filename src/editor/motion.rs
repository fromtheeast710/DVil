#[derive(Debug, Clone)]
pub enum Action {
  Insert(String),
  Delete,
  Move(Motion),
  // Copy(String),
  // Select(String, Action),
}

#[derive(Debug, Clone)]
pub enum Motion {
  Left(usize),
  Right(usize),
  Up(usize),
  Down(usize),
  // Find(u8, char),
  //
  // HopBegin(char, char)
  // HopEnd(char, char)
  //
  // WordBegin,
  // WordEnd,
  //
  // BlockBegin,
  // BlockEnd,
  //
  // Line(usize)
  // LineBegin,
  // LineEnd,
  //
  // FileBegin,
  // FileEnd,
  // BufferNext,
  // BufferPrev,
  // SplitNext,
  // SplitPrev,
}
